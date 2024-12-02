use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Pool is not initialized")]
    PoolNotInitialized,
    #[msg("Invalid LST mint")]
    InvalidLSTMint,
    #[msg("Oracle update too frequent")]
    TooFrequentUpdate,
    #[msg("Invalid oracle weights")]
    InvalidOracleWeights,
    #[msg("Math overflow occurred")]
    MathOverflow,
    #[msg("Invalid risk parameters")]
    InvalidRiskParameters,
    #[msg("Invalid risk level provided")]
    InvalidRiskLevel,
    #[msg("Invalid fee configuration")]
    InvalidFeeConfig,
    #[msg("Invalid insurance fund parameters")]
    InvalidInsuranceParams,
    #[msg("Invalid update authority")]
    InvalidAuthority,
    #[msg("Invalid validator state")]
    InvalidValidatorState,
}