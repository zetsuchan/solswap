use anchor_lang::prelude::*;

mod state;
mod contexts;
mod error;

use contexts::*;
use state::*;
use error::*;
use state::oracle::{get_weighted_median_price};

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

    /// Update oracle prices using weighted median and EMA calculation
    pub fn update_price(
        ctx: Context<UpdatePrice>
    ) -> Result<()> {
        // Get current weighted median price from oracles
        let current_price = get_weighted_median_price(
            &ctx.accounts.pyth_price_account,
            &ctx.accounts.switchboard_feed,
            &ctx.accounts.oracle_state
        )?;

        // Calculate new EMA
        let ema_price = ctx.accounts.oracle_state.calculate_ema(current_price)?;
        
        // Update oracle state
        let oracle_state = &mut ctx.accounts.oracle_state;
        oracle_state.last_price = current_price;
        oracle_state.ema_price = ema_price;
        oracle_state.update_price_history(current_price);
        oracle_state.last_update_ts = ctx.accounts.clock.unix_timestamp;

        Ok(())
    }
}