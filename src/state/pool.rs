use anchor_lang::prelude::*;
use crate::state::lst_state::LSTMetrics;

#[account]
pub struct PoolState {
    // Basic pool state
    pub is_initialized: bool,
    pub bump: u8,
    
    // LST-specific tracking
    pub lst_risk_score: u64,
    pub lst_performance_metrics: LSTMetrics,
    pub volume_24h: u64,
    pub last_update_ts: i64,
}

impl PoolState {
    // Account size calculation for space allocation
    pub const LEN: usize = 8 +  // discriminator
        1 +   // is_initialized
        1 +   // bump
        8 +   // lst_risk_score
        32 +  // lst_performance_metrics (4 * u64)
        8 +   // volume_24h
        8;    // last_update_ts

    pub fn new(bump: u8) -> Self {
        Self {
            is_initialized: true,
            bump,
            lst_risk_score: 0,
            lst_performance_metrics: LSTMetrics::default(),
            volume_24h: 0,
            last_update_ts: 0,
        }
    }
    
    pub fn update_metrics(&mut self, clock: &Sysvar<Clock>) {
        self.last_update_ts = clock.unix_timestamp;
    }
}