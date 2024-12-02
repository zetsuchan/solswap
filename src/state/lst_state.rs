use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct LSTMetrics {
    // Track MEV rewards earned by the LST
    pub mev_rewards: u64,
    // Performance score for the validator(s) backing this LST
    pub validator_performance: u64,
    // Score representing how well-distributed the stake is
    pub decentralization_score: u64,
    // Current ratio of reserves to total LST supply
    pub reserve_ratio: u64,
}

// Helper methods for LST metrics management
impl LSTMetrics {
    pub fn update_validator_performance(&mut self, new_performance: u64) {
        self.validator_performance = new_performance;
    }

    pub fn update_decentralization(&mut self, new_score: u64) {
        self.decentralization_score = new_score;
    }

    pub fn update_reserve_ratio(&mut self, new_ratio: u64) {
        self.reserve_ratio = new_ratio;
    }

    pub fn add_mev_rewards(&mut self, rewards: u64) {
        self.mev_rewards = self.mev_rewards.checked_add(rewards).unwrap_or(u64::MAX);
    }
}