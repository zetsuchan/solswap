use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::*;

#[derive(Accounts)]
pub struct RebalancePool<'info> {
    #[account(mut)]
    pub pool: Account<'info, RiskAdjustedPool>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<RebalancePool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let clock = Clock::get()?;
    
    // Check if enough time has passed since last rebalance
    require!(
        clock.unix_timestamp >= pool.last_rebalance + pool.config.rebalance_interval,
        ErrorCode::TooEarlyToRebalance
    );
    
    // Calculate new weights based on risk scores
    let new_weights = calculate_optimal_weights(
        &pool.token_weights,
        &pool.risk_scores,
        &pool.target_weights,
        pool.config.max_weight
    )?;
    
    // Update pool weights
    for (i, weight) in new_weights.iter().enumerate() {
        pool.token_weights[i].weight = *weight;
    }
    
    pool.last_rebalance = clock.unix_timestamp;

    // Implement validator redistribution for mSOL
    
    Ok(())
}

fn calculate_optimal_weights(
    current_weights: &[TokenWeight],
    risk_scores: &[RiskScore],
    target_weights: &[TargetWeight],
    max_weight: u8,
) -> Result<Vec<u16>> {
    let risk_adjusted_weights: Vec<u16> = risk_scores
        .iter()
        .map(|score| {
            let base_score = (
                score.validator_score as u16 +
                score.security_score as u16 +
                score.decentralization_score as u16 +
                score.yield_stability_score as u16
            ) / 4;

            let adjusted_score = if let Some(mev_score) = score.mev_reliability_score {
                // Adjust for JitoSOL
                (base_score + mev_score as u16) / 2
            } else if let Some(diversity_score) = score.validator_diversity_score {
                // Adjust for mSOL
                (base_score + diversity_score as u16) / 2
            } else {
                base_score
            };

            // Ensure weight does not exceed max
            adjusted_score.min(max_weight as u16 * 100)
        })
        .collect();

    Ok(risk_adjusted_weights)
}