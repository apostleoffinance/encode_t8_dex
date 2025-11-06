mod errors;
mod instructions;
mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("2wDXpw2R48Miu3PuFxMktrcx1w7QkyGa17CcB3Ev6VY7");

#[program]
pub mod encode_t8_dex {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        initialize_pool::handler(ctx)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
        add_liquidity::handler(ctx, amount_a, amount_b)
    }
}
