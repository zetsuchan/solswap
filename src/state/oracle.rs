use anchor_lang::prelude::*;
use pyth_sdk_solana::Price as PythPrice;
use switchboard_v2::AggregatorAccountData;

/// Stores oracle state including EMA calculations and historical data
#[account]
pub struct OracleState {
    /// Last recorded price from weighted oracle feeds
    pub last_price: i64,
    /// 150-second Exponential Moving Average (EMA) of price
    pub ema_price: i64,
    /// Timestamp of last price update
    pub last_update_ts: i64,
    /// Array of recent prices for EMA calculation (stores last 150 seconds)
    pub price_history: [i64; 150],
    /// Current index in price history array
    pub history_index: u8,
    /// Weight assigned to Pyth oracle (basis points)
    pub pyth_weight: u16,
    /// Weight assigned to Switchboard oracle (basis points)
    pub switchboard_weight: u16,
}

impl OracleState {
    pub const LEN: usize = 8 +  // discriminator
        8 +   // last_price
        8 +   // ema_price
        8 +   // last_update_ts
        1200 + // price_history (150 * 8)
        1 +   // history_index
        2 +   // pyth_weight
        2;    // switchboard_weight

    /// Initialize oracle state with default values
    pub fn new() -> Self {
        Self {
            last_price: 0,
            ema_price: 0,
            last_update_ts: 0,
            price_history: [0; 150],
            history_index: 0,
            pyth_weight: 5000,  // 50% weight by default
            switchboard_weight: 5000, // 50% weight by default
        }
    }

    /// Calculate exponential moving average over 150-second window
    pub fn calculate_ema(&self, current_price: i64) -> Result<i64> {
        const ALPHA: f64 = 2.0 / (150.0 + 1.0);  // Smoothing factor for 150-period EMA
        
        let ema = self.ema_price as f64 * (1.0 - ALPHA) + current_price as f64 * ALPHA;
        Ok(ema as i64)
    }

    /// Update price history array with new price
    pub fn update_price_history(&mut self, new_price: i64) {
        self.price_history[self.history_index as usize] = new_price;
        self.history_index = (self.history_index + 1) % 150;
    }
}

/// Helper functions for price calculation
pub fn get_weighted_median_price(
    pyth_price: &AccountInfo,
    switchboard_feed: &AccountInfo,
    oracle_state: &OracleState,
) -> Result<i64> {
    // Get Pyth price
    let pyth_price_data = pyth_price.try_borrow_data()?;
    let pyth_price = pyth_sdk_solana::load_price(&pyth_price_data)?;
    let pyth_price_i64 = pyth_price.agg.price;

    // Get Switchboard price
    let switchboard_state = AggregatorAccountData::new(switchboard_feed)?;
    let switchboard_result = switchboard_state.get_result()?;
    let switchboard_price_i64 = switchboard_result.mantissa;

    // Calculate weighted average
    let weighted_price = (pyth_price_i64 as i128 * oracle_state.pyth_weight as i128 +
        switchboard_price_i64 as i128 * oracle_state.switchboard_weight as i128) / 10000;

    Ok(weighted_price as i64)
}