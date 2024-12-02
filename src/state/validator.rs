use anchor_lang::prelude::*;

/// Stores performance metrics and stake information for individual validators
#[account]
pub struct ValidatorState {
    /// The validator's identity address
    pub validator_address: Pubkey,
    
    /// Rolling 30-day performance history, updated daily
    /// Each value represents performance score in basis points (0-10000)
    pub performance_history: [u64; 30],
    
    /// Current index in the performance history array
    pub current_index: u8,
    
    /// Cumulative MEV rewards earned by this validator
    pub mev_rewards: u64,
    
    /// Total amount of tokens staked with this validator
    pub total_stake: u64,
    
    /// Average APR over last 30 days in basis points
    pub average_apr: u64,
    
    /// Number of missed blocks in current epoch
    pub missed_blocks: u64,
    
    /// Timestamp of last performance update
    pub last_update: i64,
    
    /// Commission rate in basis points
    pub commission: u16,
    
    /// Authority that can update validator metrics
    pub authority: Pubkey,
    
    /// Bump seed for PDA
    pub bump: u8,
}

impl ValidatorState {
    pub const LEN: usize = 8 +    // discriminator
        32 +   // validator_address
        240 +  // performance_history (30 * 8)
        1 +    // current_index
        8 +    // mev_rewards
        8 +    // total_stake
        8 +    // average_apr
        8 +    // missed_blocks
        8 +    // last_update
        2 +    // commission
        32 +   // authority
        1;     // bump

    /// Calculates the average performance over the last 30 days
    pub fn calculate_average_performance(&self) -> u64 {
        let mut sum = 0u64;
        let mut count = 0u64;
        
        for score in self.performance_history.iter() {
            if *score > 0 {
                sum = sum.checked_add(*score).unwrap_or(u64::MAX);
                count = count.checked_add(1).unwrap_or(u64::MAX);
            }
        }
        
        if count == 0 {
            return 0;
        }
        
        sum.checked_div(count).unwrap_or(0)
    }

    /// Updates rolling performance metrics and calculates risk score
    pub fn update_metrics(
        &mut self,
        new_performance: u64,
        new_mev_reward: u64,
        current_timestamp: i64,
    ) -> Result<u64> {
        // Ensure we don't update more than once per day
        require!(
            self.last_update == 0 || 
            current_timestamp.checked_sub(self.last_update).unwrap() >= 86400,
            ErrorCode::TooFrequentUpdate
        );

        // Update performance history
        self.performance_history[self.current_index as usize] = new_performance;
        self.current_index = ((self.current_index as u16 + 1) % 30) as u8;

        // Update MEV rewards
        self.mev_rewards = self.mev_rewards
            .checked_add(new_mev_reward)
            .ok_or(ErrorCode::MathOverflow)?;

        // Update timestamp
        self.last_update = current_timestamp;

        // Calculate risk score based on multiple factors
        self.calculate_risk_score()
    }

    /// Calculates a risk score based on validator metrics
    fn calculate_risk_score(&self) -> Result<u64> {
        let avg_performance = self.calculate_average_performance();
        
        // Weight different factors for risk calculation
        let performance_score = avg_performance
            .checked_mul(6000)
            .ok_or(ErrorCode::MathOverflow)?;
            
        let commission_score = (10000u64
            .checked_sub(self.commission as u64)
            .ok_or(ErrorCode::MathOverflow)?)
            .checked_mul(2000)
            .ok_or(ErrorCode::MathOverflow)?;
            
        let mev_score = if self.mev_rewards > 0 {
            2000u64
        } else {
            0u64
        };

        let total_score = performance_score
            .checked_add(commission_score)
            .ok_or(ErrorCode::MathOverflow)?
            .checked_add(mev_score)
            .ok_or(ErrorCode::MathOverflow)?;

        Ok(total_score.checked_div(10000).unwrap_or(0))
    }
}
