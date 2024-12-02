use anchor_lang::prelude::*;
use crate::state::{pool::PoolState, insurance_fund::InsuranceFund};
use crate::state::liquidity_protection::*;

#[derive(Accounts)]
pub struct LiquidityProtection<'info> {
    #[account(mut)]
    pub pool_state: Account<'info, PoolState>,

    #[account(mut)]
    pub insurance_fund: Account<'info, InsuranceFund>,

    pub clock: Sysvar<'info, Clock>,
}

impl<'info> LiquidityProtection<'info> {
    /// Updates all liquidity protection parameters based on current state
    pub fn update_liquidity_parameters(&mut self) -> Result<()> {
        // Calculate current utilization
        let utilization = calculate_pool_utilization(
            self.pool_state.total_deposits,
            self.pool_state.total_borrows,
        )?;

        // Assess current risk level
        let risk_level = assess_current_risk(
            &self.pool_state,
            utilization,
        )?;

        // Update dynamic fee rate
        self.pool_state.fee_rate = calculate_dynamic_fee(
            utilization,
            risk_level,
            self.pool_state.base_fee,
        )?;

        // Update insurance fund allocation
        self.insurance_fund.update_allocation(risk_level)?;

        // Update last update timestamp
        self.insurance_fund.last_rebalance = self.clock.unix_timestamp;

        Ok(())
    }
}
