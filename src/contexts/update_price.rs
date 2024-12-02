use anchor_lang::prelude::*;
use crate::state::{pool::PoolState, oracle::OracleState};

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut)]
    pub pool: Account<'info, PoolState>,

    #[account(
        mut,
        constraint = oracle_state.last_update_ts < Clock::get()?.unix_timestamp - 3 @ ErrorCode::TooFrequentUpdate
    )]
    pub oracle_state: Account<'info, OracleState>,

    /// CHECK: Verified in price calculation logic
    pub pyth_price_account: AccountInfo<'info>,

    /// CHECK: Verified in price calculation logic
    pub switchboard_feed: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>,
}
