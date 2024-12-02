use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Pool is not initialized")]
    PoolNotInitialized,
    #[msg("Invalid LST mint")]
    InvalidLSTMint,
}