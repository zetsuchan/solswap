use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    /// Pool authority PDA
    pub authority: Pubkey,
    /// Bump seed for authority PDA
    pub bump: u8,
    /// LST tokens supported in this pool
    pub tokens: Vec<Pubkey>,
    /// amplification coefficient (A)
    pub amp: u64,
    /// fee percentage in basis points
    pub fee_bps: u16,
    /// admin fee percentage in basis points
    pub admin_fee_bps: u16,
    /// total LP token supply
    pub lp_supply: u64,
}

#[account]
pub struct UserPosition {
    /// Owner of the position
    pub owner: Pubkey,
    /// Pool this position belongs to
    pub pool: Pubkey,
    /// Amount of LP tokens owned
    pub lp_tokens: u64,
}
