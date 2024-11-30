// programs/solswap/src/state/liquid_stake_pool.rs
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LiquidStakePool {
    pub token_mint: Pubkey,
    pub reserve_authority: Pubkey,
    pub lending_pool: Option<Pubkey>,
    pub stake_rate: StakeRate,
    pub rewards_tracking: RewardsTracking,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct StakeRate {
    pub current_rate: u64,
    pub last_update: i64,
    pub historical_rates: Vec<RateSnapshot>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RateSnapshot {
    pub timestamp: i64,
    pub rate: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RewardsTracking {
    pub total_rewards: u64,
    pub last_claimed: i64,
}