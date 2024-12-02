use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use crate::state::pool::PoolState;

#[derive(Accounts)]
pub struct LSTPool<'info> {
    #[account(mut)]
    pub pool_state: Box<Account<'info, PoolState>>,
    
    #[account(
        constraint = lst_mint.decimals == 6,
    )]
    pub lst_mint: Account<'info, Mint>,
    
    #[account(
        constraint = usdc_mint.decimals == 6,
    )]
    pub usdc_mint: Account<'info, Mint>,
    
    /// CHECK: This is the pool authority
    pub authority: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> LSTPool<'info> {
    pub fn validate(&self) -> Result<()> {
        require!(self.pool_state.is_initialized, ErrorCode::PoolNotInitialized);
        Ok(())
    }
}