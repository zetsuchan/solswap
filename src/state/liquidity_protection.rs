use anchor_lang::prelude::*;
use crate::state::pool::PoolState;
use crate::state::insurance_fund::InsuranceFund;

/// Constants for liquidity protection calculations
pub const BASE_FEE_RATE: u64 = 30;           // 0.3% base fee in basis points
pub const MIN_FEE_RATE: u64 = 10;            // 0.1% minimum fee
pub const MAX_FEE_RATE: u64 = 100;           // 1% maximum fee
pub const OPTIMAL_UTILIZATION: u64 = 8_000;   // 80% optimal utilization
pub const SLOPE_1: u64 = 20;                 // Rate of increase below optimal
pub const SLOPE_2: u64 = 100;                // Rate of increase above optimal

/// Calculates current pool utilization in basis points (0-10000)
pub fn calculate_pool_utilization(
    total_deposits: u64,
    total_borrows: u64,
) -> Result<u64> {
    if total_deposits == 0 {
        return Ok(0);
    }

    Ok(((total_borrows as u128 * 10_000) / total_deposits as u128) as u64)
}

/// Assesses current risk level based on various metrics
pub fn assess_current_risk(
    pool_state: &PoolState,
    utilization: u64,
) -> Result<u16> {
    // Weight different risk factors
    let utilization_risk = if utilization > OPTIMAL_UTILIZATION {
        // Higher risk when utilization exceeds optimal
        ((utilization - OPTIMAL_UTILIZATION) * 60) / (10_000 - OPTIMAL_UTILIZATION)
    } else {
        // Lower risk below optimal utilization
        (utilization * 40) / OPTIMAL_UTILIZATION
    };

    // Consider LST-specific risks
    let lst_risk = ((10_000 - pool_state.lst_performance_metrics.validator_performance) * 40) / 10_000;

    let total_risk = (utilization_risk + lst_risk)
        .min(10_000) as u16;

    Ok(total_risk)
}

/// Calculates dynamic fee based on utilization and risk level
pub fn calculate_dynamic_fee(
    utilization: u64,
    risk_level: u16,
    base_fee: u64,
) -> Result<u64> {
    let util_multiplier = if utilization <= OPTIMAL_UTILIZATION {
        // Below optimal: gradual increase
        BASE_FEE_RATE + (utilization * SLOPE_1) / OPTIMAL_UTILIZATION
    } else {
        // Above optimal: steeper increase
        let excess_util = utilization - OPTIMAL_UTILIZATION;
        BASE_FEE_RATE + SLOPE_1 + 
            (excess_util * SLOPE_2) / (10_000 - OPTIMAL_UTILIZATION)
    };

    // Apply risk adjustment
    let risk_multiplier = 10_000 + (risk_level as u64 * 5_000) / 10_000;
    let fee_rate = (util_multiplier * risk_multiplier) / 10_000;

    // Ensure fee is within bounds
    Ok(fee_rate.clamp(MIN_FEE_RATE, MAX_FEE_RATE))
}
