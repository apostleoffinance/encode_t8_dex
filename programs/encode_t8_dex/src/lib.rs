pub mod errors;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use errors::ErrorCode;

declare_id!("2wDXpw2R48Miu3PuFxMktrcx1w7QkyGa17CcB3Ev6VY7");

#[program]
pub mod encode_t8_dex {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        pool.mint_a = ctx.accounts.mint_a.key();
        pool.mint_b = ctx.accounts.mint_b.key();
        pool.token_vault_a = ctx.accounts.token_vault_a.key();
        pool.token_vault_b = ctx.accounts.token_vault_b.key();

        msg!(
            "Pool initialized for mints: {} and {}",
            pool.mint_a,
            pool.mint_b
        );

        Ok(())
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
        if amount_a == 0 || amount_b == 0 {
            // TODO: more logic
            return err!(ErrorCode::ZeroAmount);
        }
        token::transfer(ctx.accounts.transfer_a_context(), amount_a)?;

        token::transfer(ctx.accounts.transfer_b_context(), amount_b)?;

        msg!(
            "Liquidity added: {} of token A, {} of token B",
            amount_a,
            amount_b
        );

        Ok(())
    }
}

#[account]
pub struct Pool {
    // Token A public key
    pub mint_a: Pubkey,
    // Token B public key
    pub mint_b: Pubkey,
    // Account holding A tokens
    pub token_vault_a: Pubkey,
    // Account holding B tokens
    pub token_vault_b: Pubkey,
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    // Account containing data about pool
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 32 + 32 + 32 // 8 byte for Anchor + 32 x 4 Pubkey
    )]
    pub pool: Account<'info, Pool>,

    // Token accounts
    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,

    // Account that holds token A
    #[account(
        init,
        payer = payer,
        token::mint = mint_a,
        token::authority = pool // PDA
    )]
    pub token_vault_a: Account<'info, TokenAccount>,
    // Account that holds token B
    #[account(
        init,
        payer = payer,
        token::mint = mint_b,
        token::authority = pool
    )]
    pub token_vault_b: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    // Pool account where liquidity is being added to
    pub pool: Account<'info, Pool>,

    // Token vaults
    #[account(
        mut,
        // check that this is really the address stored in the pool account
        address = pool.token_vault_a
    )]
    pub token_vault_a: Account<'info, TokenAccount>,
    #[account(
        mut,
        address = pool.token_vault_b
    )]
    pub token_vault_b: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    // User token accounts
    #[account(mut)]
    pub user_token_account_a: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account_b: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
