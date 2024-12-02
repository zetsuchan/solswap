use anchor_lang::prelude::*;

/// LSTMetrics tracks performance and risk metrics for Liquid Staking Tokens
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct LSTMetrics {
    // Track MEV rewards earned by the LST validators
    pub mev_rewards: u64,
    // Aggregate performance score for validators backing this LST
    pub validator_performance: u64,
    // Score measuring stake distribution across validators
    pub decentralization_score: u64,
    // Current ratio of reserves to total LST supply (basis points)
    pub reserve_ratio: u64,
}

impl LSTMetrics {
    /// Updates validator performance score
    pub fn update_validator_performance(&mut self, new_performance: u64) {
        self.validator_performance = new_performance;
    }

    /// Updates decentralization score based on stake distribution
    pub fn update_decentralization(&mut self, new_score: u64) {
        self.decentralization_score = new_score;
    }

    /// Updates reserve ratio (in basis points)
    pub fn update_reserve_ratio(&mut self, new_ratio: u64) {
        self.reserve_ratio = new_ratio;
    }

    /// Safely adds MEV rewards with overflow protection
    pub fn add_mev_rewards(&mut self, rewards: u64) {
        self.mev_rewards = self.mev_rewards.checked_add(rewards).unwrap_or(u64::MAX);
    }
}