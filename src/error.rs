use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Pool is not initialized")]
    PoolNotInitialized,
    #[msg("Invalid LST mint")]
    InvalidLSTMint,
    #[msg("Oracle update too frequent")]
    TooFrequentUpdate,
    #[msg("Invalid oracle weight configuration")]
    InvalidOracleWeights,
}
