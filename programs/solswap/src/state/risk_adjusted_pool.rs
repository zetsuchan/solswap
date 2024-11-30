use anchor_lang::prelude::*;

#[account]
pub struct RiskAdjustedPool {
    /// Pool authority PDA
    pub authority: Pubkey,
    /// Pool configuration
    pub config: PoolConfig,
    /// LST tokens and their weights
    pub token_weights: Vec<TokenWeight>,
    /// Last rebalance timestamp
    pub last_rebalance: i64,
    /// Target weights for each LST
    pub target_weights: Vec<TargetWeight>,
    /// Risk scores for each LST
    pub risk_scores: Vec<RiskScore>,
    // Include new fields
    pub jito_reserves: Option<JitoReserves>,
    pub msol_pool: Option<LiquidStakePool>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PoolConfig {
    /// Maximum weight for any single LST
    pub max_weight: u8,
    /// Minimum time between rebalances
    pub rebalance_interval: i64,
    /// Maximum deviation from target weights
    pub max_weight_deviation: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TokenWeight {
    /// Token mint address
    pub mint: Pubkey,
    /// Current weight (basis points)
    pub weight: u16,
    /// Amount of tokens in pool
    pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TargetWeight {
    /// Token mint address
    pub mint: Pubkey,
    /// Target weight (basis points)
    pub target: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RiskScore {
    /// Token mint address
    pub mint: Pubkey,
    /// Validator performance score (0-100)
    pub validator_score: u8,
    /// Protocol security score (0-100)
    pub security_score: u8,
    /// Decentralization score (0-100)
    pub decentralization_score: u8,
    /// Yield stability score (0-100)
    pub yield_stability_score: u8,
    // New fields
    pub mev_reliability_score: Option<u8>,      // For JitoSOL
    pub validator_diversity_score: Option<u8>,  // For mSOL
}