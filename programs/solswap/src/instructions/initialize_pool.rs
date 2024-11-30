use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(init, payer = authority, space = 8 + 32 + 1 + 32 * 10 + 8 + 2 + 2 + 8)]
    pub pool: Account<'info, Pool>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializePool>,
    amp: u64,
    fee_bps: u16,
    admin_fee_bps: u16,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    
    pool.authority = ctx.accounts.authority.key();
    pool.amp = amp;
    pool.fee_bps = fee_bps;
    pool.admin_fee_bps = admin_fee_bps;
    pool.lp_supply = 0;
    
    Ok(())
}
