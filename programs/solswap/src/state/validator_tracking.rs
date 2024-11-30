use anchor_lang::prelude::*;

#[account]
pub struct ValidatorInfo {
    /// Validator identity
    pub identity: Pubkey,
    /// Current stake amount
    pub stake_amount: u64,
    /// Performance metrics
    pub metrics: ValidatorMetrics,
    /// LST protocols using this validator
    pub lst_protocols: Vec<LstProtocolInfo>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ValidatorMetrics {
    /// Average APY over last 30 days
    pub avg_apy: u64,
    /// Uptime percentage (basis points)
    pub uptime_bps: u16,
    /// Number of epochs active
    pub epochs_active: u64,
    /// Last updated slot
    pub last_update: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LstProtocolInfo {
    /// Protocol identifier
    pub protocol: Pubkey,
    /// Amount staked through this protocol
    pub staked_amount: u64,
    /// Protocol-specific risk score
    pub risk_score: u8,
}

#[account]
pub struct ReservesTracker {
    /// Authority for updating reserves
    pub authority: Pubkey,
    /// Last update timestamp
    pub last_update: i64,
    /// Total SOL in reserves
    pub total_reserves: u64,
    /// Breakdown by LST protocol
    pub protocol_reserves: Vec<ProtocolReserves>,
    /// Add JitoSOL-specific reserves
    pub jito_mev_rewards: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ProtocolReserves {
    /// Protocol identifier
    pub protocol: Pubkey,
    /// Total SOL backing LSTs
    pub sol_reserves: u64,
    /// Total LSTs issued
    pub lst_supply: u64,
    /// Last verification timestamp
    pub last_verified: i64,
}
