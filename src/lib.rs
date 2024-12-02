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

    // ... (previous functions remain the same)

    /// Initialize a new validator state account
    pub fn initialize_validator(
        ctx: Context<InitializeValidator>,
        validator_address: Pubkey,
        bump: u8,
    ) -> Result<()> {
        let validator_state = &mut ctx.accounts.validator_state;
        
        *validator_state = ValidatorState {
            validator_address,
            performance_history: [0; 30],
            current_index: 0,
            mev_rewards: 0,
            total_stake: 0,
            average_apr: 0,
            missed_blocks: 0,
            last_update: 0,
            commission: 0,
            authority: ctx.accounts.authority.key(),
            bump,
        };
        
        Ok(())
    }

    /// Update validator metrics and recalculate risk scores
    pub fn update_validator_metrics(
        ctx: Context<UpdateValidatorMetrics>,
        new_performance: u64,
        new_mev_reward: u64,
    ) -> Result<()> {
        ctx.accounts.process_update(new_performance, new_mev_reward)
    }
}

#[derive(Accounts)]
pub struct InitializeValidator<'info> {
    #[account(init, payer = authority, space = ValidatorState::LEN)]
    pub validator_state: Account<'info, ValidatorState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
