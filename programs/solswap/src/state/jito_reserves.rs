// programs/solswap/src/state/jito_reserves.rs
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct JitoReserves {
    pub base_staking_reserves: u64,
    pub mev_rewards_reserves: u64,
    pub last_mev_distribution: i64,
    pub validator_performance: Vec<JitoValidatorMetrics>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct JitoValidatorMetrics {
    pub validator_identity: Pubkey,
    pub mev_earned: u64,
    pub performance_score: u8,
}