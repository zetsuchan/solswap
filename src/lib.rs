use anchor_lang::prelude::*;

mod state;
mod contexts;
mod error;

use contexts::*;
use state::*;
use error::*;

declare_id!("your_program_id_here");

#[program]
pub mod solswap {
    use super::*;
    
    /// Initialize a new LST pool with default metrics
    pub fn initialize_lst_pool(
        ctx: Context<LSTPool>,
        bump: u8,
    ) -> Result<()> {
        let pool_state = &mut ctx.accounts.pool_state;
        *pool_state = PoolState::new(bump);
        Ok(())
    }
    
    /// Update LST pool metrics
    pub fn update_lst_metrics(
        ctx: Context<LSTPool>,
        new_performance: u64,
        new_decentralization: u64,
        new_reserve_ratio: u64,
    ) -> Result<()> {
        let pool_state = &mut ctx.accounts.pool_state;
        
        pool_state.lst_performance_metrics.update_validator_performance(new_performance);
        pool_state.lst_performance_metrics.update_decentralization(new_decentralization);
        pool_state.lst_performance_metrics.update_reserve_ratio(new_reserve_ratio);
        
        // Update timestamp
        let clock = Clock::get()?;
        pool_state.update_metrics(&clock);
        
        Ok(())
    }
}