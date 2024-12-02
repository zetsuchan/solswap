use anchor_lang::prelude::*;
use crate::state::{pool::PoolState, validator::ValidatorState};

/// Context for updating validator metrics
#[derive(Accounts)]
pub struct UpdateValidatorMetrics<'info> {
    /// The validator state account to update
    #[account(
        mut,
        seeds = [b"validator", validator_state.validator_address.as_ref()],
        bump = validator_state.bump,
    )]
    pub validator_state: Account<'info, ValidatorState>,

    /// The LST pool state that this validator contributes to
    #[account(mut)]
    pub pool_state: Account<'info, PoolState>,

    /// Authority allowed to update validator metrics
    pub authority: Signer<'info>,

    /// System clock for timestamp checks
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> UpdateValidatorMetrics<'info> {
    pub fn process_update(
        &mut self,
        new_performance: u64,
        new_mev_reward: u64,
    ) -> Result<()> {
        // Verify update authority
        require!(
            self.validator_state.authority == self.authority.key(),
            ErrorCode::InvalidAuthority
        );

        // Update validator metrics and get new risk score
        let new_risk_score = self.validator_state.update_metrics(
            new_performance,
            new_mev_reward,
            self.clock.unix_timestamp,
        )?;

        // Update pool's aggregate validator metrics
        self.update_pool_metrics(new_risk_score)?;

        Ok(())
    }

    /// Updates pool-level metrics based on validator performance
    fn update_pool_metrics(&mut self, new_risk_score: u64) -> Result<()> {
        let pool_state = &mut self.pool_state;
        
        // Update validator performance in pool metrics
        pool_state.lst_performance_metrics.validator_performance = 
            self.calculate_weighted_validator_performance(new_risk_score)?;

        // Update MEV rewards tracking
        pool_state.lst_performance_metrics.mev_rewards = 
            self.validator_state.mev_rewards;

        Ok(())
    }

    /// Calculates weighted validator performance based on stake
    fn calculate_weighted_validator_performance(&self, risk_score: u64) -> Result<u64> {
        // For now, we just return the risk score directly
        // In a full implementation, this would weight multiple validators' scores
        // based on their relative stake in the LST pool
        Ok(risk_score)
    }
}
