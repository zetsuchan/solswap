use anchor_lang::prelude::*;

#[error_code]
pub enum SwapError {
    #[msg("Invalid pool parameters")]
    InvalidPoolParameters,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Invalid calculation")]
    InvalidCalculation,
}
