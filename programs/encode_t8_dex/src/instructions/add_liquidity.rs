use crate::errors::ErrorCode;

use crate::state::Pool;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

pub fn handler(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
    if amount_a == 0 || amount_b == 0 {
        return err!(ErrorCode::ZeroAmount);
    }

    // TODO: more logic to determine lp amount to mint
    let lp_amount_to_mint = amount_a;

    token::transfer(ctx.accounts.transfer_a_context(), amount_a)?;
    token::transfer(ctx.accounts.transfer_b_context(), amount_b)?;

    token::mint_to(ctx.accounts.mint_lp_context(), lp_amount_to_mint)?;

    msg!(
        "Liquidity added: {} of token A, {} of token B.",
        amount_a,
        amount_b
    );
    msg!("Minted {} LP tokens.", lp_amount_to_mint);

    Ok(())
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(
        seeds = [b"pool", pool.mint_a.as_ref(), pool.mint_b.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        address = pool.token_vault_a
    )]
    pub token_vault_a: Account<'info, TokenAccount>,
    #[account(
        mut,
        address = pool.token_vault_b
    )]
    pub token_vault_b: Account<'info, TokenAccount>,

    #[account(
        mut,
        address = pool.lp_mint
    )]
    pub lp_mint: Account<'info, Mint>,

    // Function caller
    #[account(mut)]
    pub user: Signer<'info>,

    // Счета пользователя для токенов A и B
    #[account(mut)]
    pub user_token_account_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account_b: Account<'info, TokenAccount>,

    // User token account for LP tokens
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = lp_mint,
        associated_token::authority = user,
    )]
    pub user_lp_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

// Functions for CPI context creation
impl<'info> AddLiquidity<'info> {
    pub fn transfer_a_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_account_a.to_account_info(),
                to: self.token_vault_a.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    pub fn transfer_b_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_token_account_b.to_account_info(),
                to: self.token_vault_b.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    pub fn mint_lp_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.lp_mint.to_account_info(),
                to: self.user_lp_token_account.to_account_info(),
                authority: self.pool.to_account_info(), // Просто передаем PDA
            },
        )
    }
}
