use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

#[derive(Accounts)]
pub struct CompoundRewards<'info> {
    #[account(mut)]
    pub pool: Account<'info, RiskAdjustedPool>,
    
    #[account(mut)]
    pub reserves_tracker: Account<'info, ReservesTracker>,
    
    #[account(
        mut,
        constraint = rewards_vault.owner == pool.key()
    )]
    pub rewards_vault: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RewardsSnapshot {
    pub timestamp: i64,
    pub staking_rewards: u64,
    pub mev_rewards: u64,
}

pub fn handler(ctx: Context<CompoundRewards>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let reserves = &mut ctx.accounts.reserves_tracker;
    
    // Verify minimum time between compounds
    let now = Clock::get()?.unix_timestamp;
    require!(
        now >= reserves.last_update + pool.config.min_compound_interval,
        SwapError::TooEarlyToCompound
    );

    // Calculate new rewards since last compound
    let new_staking_rewards = calculate_staking_rewards(
        reserves.total_reserves,
        reserves.last_update,
        now,
    )?;
    
    let new_mev_rewards = calculate_mev_rewards(
        &pool.jito_reserves.as_ref().unwrap(),
        reserves.last_update,
        now,
    )?;

    // Update reserves with compounded amounts
    reserves.total_reserves = reserves
        .total_reserves
        .checked_add(new_staking_rewards)
        .ok_or(SwapError::MathOverflow)?;
        
    reserves.jito_mev_rewards = reserves
        .jito_mev_rewards
        .checked_add(new_mev_rewards)
        .ok_or(SwapError::MathOverflow)?;

    // Store rewards snapshot
    reserves.rewards_history.push(RewardsSnapshot {
        timestamp: now,
        staking_rewards: new_staking_rewards,
        mev_rewards: new_mev_rewards,
    });

    reserves.last_update = now;

    Ok(())
}

fn calculate_staking_rewards(
    total_reserves: u64,
    last_update: i64,
    current_time: i64,
) -> Result<u64> {
    // Annual staking rate ~6%
    let annual_rate = 600;
    let time_elapsed = (current_time - last_update) as u64;
    
    let rewards = (total_reserves
        .checked_mul(annual_rate)
        .ok_or(SwapError::MathOverflow)?
        .checked_mul(time_elapsed)
        .ok_or(SwapError::MathOverflow)?)
        .checked_div(365 * 24 * 60 * 60 * 10000)
        .ok_or(SwapError::MathOverflow)?;
        
    Ok(rewards)
}

fn calculate_mev_rewards(
    jito_reserves: &JitoReserves,
    last_update: i64, 
    current_time: i64,
) -> Result<u64> {
    // Calculate MEV rewards based on validator performance
    let mev_rate = jito_reserves.validator_performance.iter()
        .map(|v| v.mev_earned)
        .sum::<u64>();
        
    let time_elapsed = (current_time - last_update) as u64;
    
    let rewards = (mev_rate
        .checked_mul(time_elapsed)
        .ok_or(SwapError::MathOverflow)?)
        .checked_div(24 * 60 * 60)
        .ok_or(SwapError::MathOverflow)?;
        
    Ok(rewards)
}