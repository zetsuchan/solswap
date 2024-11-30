// programs/solswap/src/state/lst_pool_config.rs
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LstPoolConfig {
    pub jito_config: JitoPoolSettings,
    pub msol_config: MsolPoolSettings,
    pub rebalance_threshold: u64,
    pub fee_structure: FeeStructure,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct JitoPoolSettings {
    pub mev_share_fee: u64,
    // Other settings...
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MsolPoolSettings {
    pub directed_stake_enabled: bool,
    // Other settings...
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FeeStructure {
    pub swap_fee: u64,
    pub admin_fee: u64,
    pub mev_share_fee: u64,
}