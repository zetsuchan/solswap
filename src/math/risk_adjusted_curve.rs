use anchor_lang::prelude::*;
use crate::state::pool::PoolState;

/// Constants for risk adjustment calculations
pub const BASE_RISK_MULTIPLIER: u64 = 10_000; // 1.0 in basis points
pub const MIN_RISK_MULTIPLIER: u64 = 5_000;   // 0.5 in basis points
pub const MAX_RISK_MULTIPLIER: u64 = 15_000;  // 1.5 in basis points

/// Weights for different risk components (must sum to 10000)
pub const VALIDATOR_PERFORMANCE_WEIGHT: u64 = 4_000;  // 40%
pub const RESERVE_RATIO_WEIGHT: u64 = 4_000;         // 40%
pub const DECENTRALIZATION_WEIGHT: u64 = 2_000;      // 20%

/// Calculates the adjusted output amount for a swap, incorporating risk metrics
pub fn calculate_swap_with_risk_adjustment(
    amount_in: u64,
    pool_state: &PoolState,
    base_curve_calculator: impl Fn(u64) -> Result<u64>,
) -> Result<u64> {
    // First calculate base swap amount using standard curve
    let base_amount = base_curve_calculator(amount_in)?;
    
    // Calculate risk multiplier based on current pool metrics
    let risk_multiplier = calculate_risk_multiplier(
        pool_state.lst_performance_metrics.validator_performance,
        pool_state.lst_performance_metrics.reserve_ratio,
        pool_state.lst_performance_metrics.decentralization_score,
    )?;
    
    // Apply risk adjustment to base amount
    let adjusted_amount = apply_risk_multiplier(base_amount, risk_multiplier)?;
    
    Ok(adjusted_amount)
}

/// Calculates a risk multiplier based on various LST metrics
/// Returns a multiplier in basis points (10000 = 1.0)
pub fn calculate_risk_multiplier(
    validator_performance: u64,
    reserve_ratio: u64,
    decentralization_score: u64,
) -> Result<u64> {
    // Normalize each metric to a 0-10000 scale
    let normalized_performance = normalize_metric(validator_performance)?;
    let normalized_reserves = normalize_metric(reserve_ratio)?;
    let normalized_decentralization = normalize_metric(decentralization_score)?;
    
    // Calculate weighted components
    let performance_component = normalized_performance
        .checked_mul(VALIDATOR_PERFORMANCE_WEIGHT)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(10_000)
        .ok_or(ErrorCode::MathOverflow)?;
        
    let reserve_component = normalized_reserves
        .checked_mul(RESERVE_RATIO_WEIGHT)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(10_000)
        .ok_or(ErrorCode::MathOverflow)?;
        
    let decentralization_component = normalized_decentralization
        .checked_mul(DECENTRALIZATION_WEIGHT)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(10_000)
        .ok_or(ErrorCode::MathOverflow)?;
    
    // Combine components and scale to final multiplier
    let base_score = performance_component
        .checked_add(reserve_component)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_add(decentralization_component)
        .ok_or(ErrorCode::MathOverflow)?;
    
    // Convert base score to multiplier range
    let multiplier_range = MAX_RISK_MULTIPLIER
        .checked_sub(MIN_RISK_MULTIPLIER)
        .ok_or(ErrorCode::MathOverflow)?;
    
    let risk_multiplier = MIN_RISK_MULTIPLIER
        .checked_add(
            base_score
                .checked_mul(multiplier_range)
                .ok_or(ErrorCode::MathOverflow)?
                .checked_div(10_000)
                .ok_or(ErrorCode::MathOverflow)?
        )
        .ok_or(ErrorCode::MathOverflow)?;
    
    Ok(risk_multiplier)
}

/// Normalizes a metric to a 0-10000 scale (basis points)
fn normalize_metric(metric: u64) -> Result<u64> {
    if metric > 10_000 {
        return Ok(10_000);
    }
    Ok(metric)
}

/// Applies a risk multiplier (in basis points) to an amount
fn apply_risk_multiplier(amount: u64, multiplier: u64) -> Result<u64> {
    amount
        .checked_mul(multiplier)
        .ok_or(ErrorCode::MathOverflow)?
        .checked_div(BASE_RISK_MULTIPLIER)
        .ok_or(ErrorCode::MathOverflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_multiplier_calculation() {
        // Test case with perfect metrics
        let perfect_multiplier = calculate_risk_multiplier(
            10_000, // Perfect validator performance
            10_000, // Perfect reserve ratio
            10_000, // Perfect decentralization
        ).unwrap();
        assert_eq!(perfect_multiplier, MAX_RISK_MULTIPLIER);

        // Test case with minimum metrics
        let min_multiplier = calculate_risk_multiplier(
            0,  // Minimum validator performance
            0,  // Minimum reserve ratio
            0,  // Minimum decentralization
        ).unwrap();
        assert_eq!(min_multiplier, MIN_RISK_MULTIPLIER);

        // Test case with mixed metrics
        let mixed_multiplier = calculate_risk_multiplier(
            5_000,  // 50% validator performance
            5_000,  // 50% reserve ratio
            5_000,  // 50% decentralization
        ).unwrap();
        assert_eq!(mixed_multiplier, 10_000);  // Should be base multiplier
    }
}
