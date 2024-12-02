use anchor_lang::prelude::*;

/// Maintains the state of the insurance fund that protects against unexpected events
#[account]
pub struct InsuranceFund {
    /// Total assets held in the insurance fund
    pub total_assets: u64,
    /// Target allocation as a percentage of total pool liquidity (basis points)
    pub target_allocation: u16,
    /// Minimum allocation required (basis points)
    pub minimum_allocation: u16,
    /// Maximum allocation allowed (basis points)
    pub maximum_allocation: u16,
    /// Current risk level assessment (0-10000)
    pub current_risk_level: u16,
    /// Last time the fund was rebalanced
    pub last_rebalance: i64,
    /// Amount of funds deployed for protection
    pub deployed_amount: u64,
    /// Authority that can adjust fund parameters
    pub authority: Pubkey,
    /// Bump seed for PDA
    pub bump: u8,
}

impl InsuranceFund {
    pub const LEN: usize = 8 +    // discriminator
        8 +    // total_assets
        2 +    // target_allocation
        2 +    // minimum_allocation
        2 +    // maximum_allocation
        2 +    // current_risk_level
        8 +    // last_rebalance
        8 +    // deployed_amount
        32 +   // authority
        1;     // bump

    /// Initialize a new insurance fund with default parameters
    pub fn new(authority: Pubkey, bump: u8) -> Self {
        Self {
            total_assets: 0,
            target_allocation: 500,     // 5% default target
            minimum_allocation: 200,     // 2% minimum
            maximum_allocation: 1000,    // 10% maximum
            current_risk_level: 0,
            last_rebalance: 0,
            deployed_amount: 0,
            authority,
            bump,
        }
    }

    /// Updates the insurance fund allocation based on current risk level
    pub fn update_allocation(&mut self, risk_level: u16) -> Result<()> {
        require!(risk_level <= 10_000, ErrorCode::InvalidRiskLevel);
        
        // Adjust target allocation based on risk level
        let range = self.maximum_allocation.saturating_sub(self.minimum_allocation);
        let risk_adjusted_addition = (range as u32 * risk_level as u32 / 10_000) as u16;
        self.target_allocation = self.minimum_allocation.saturating_add(risk_adjusted_addition);
        
        self.current_risk_level = risk_level;
        Ok(())
    }
}
