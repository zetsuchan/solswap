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

    /// Updates liquidity protection parameters
    pub fn update_protection_params(ctx: Context<LiquidityProtection>) -> Result<()> {
        ctx.accounts.update_liquidity_parameters()
    }

    /// Initialize insurance fund
    pub fn initialize_insurance_fund(
        ctx: Context<InitializeInsuranceFund>,
        bump: u8,
    ) -> Result<()> {
        let insurance_fund = &mut ctx.accounts.insurance_fund;
        *insurance_fund = InsuranceFund::new(ctx.accounts.authority.key(), bump);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeInsuranceFund<'info> {
    #[account(init, payer = authority, space = InsuranceFund::LEN)]
    pub insurance_fund: Account<'info, InsuranceFund>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
