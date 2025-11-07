use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Amount must be greater than zero.")]
    ZeroAmount,
    #[msg("Deposited token amounts do not match the pool ratio.")]
    InvalidRatio,
    #[msg("Slippage exceeded.")]
    SlippageExceeded,
    #[msg("An error occurred during calculation.")]
    CalculationFailure,
}
