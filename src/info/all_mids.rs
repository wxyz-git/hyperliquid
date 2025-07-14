//! Get mid prices for all trading pairs
//! 
//! This module provides functionality to retrieve current mid prices for all available
//! trading pairs on HyperLiquid.

use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::Deserialize;

use crate::client::HyperLiquidClient;

/// A price value that deserializes from JSON string to Decimal
#[derive(Deserialize, Debug, Clone)]
pub struct Price(#[serde(with = "rust_decimal::serde::str")] pub Decimal);

/// Mid prices for all trading pairs
/// 
/// Contains a mapping of coin symbols (e.g., "BTC", "ETH") to their current mid prices
/// as strings to preserve exact decimal precision.
#[derive(Deserialize, Debug)]
pub struct MidPrices {
    #[serde(flatten)]
    pub prices: HashMap<String, Price>,
}

impl HyperLiquidClient {
    /// Get current mid prices for all trading pairs
    /// 
    /// Returns a mapping of all available trading pairs to their current mid prices.
    /// Mid prices are calculated as the average of the best bid and ask prices.
    /// 
    /// # Returns
    /// 
    /// A [`MidPrices`] struct containing a HashMap of coin symbols to price strings.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use hyperliquid::client::HyperLiquidClient;
    /// 
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let client = HyperLiquidClient::new();
    ///     let mid_prices = client.get_all_mids().await?;
    ///     
    ///     if let Some(btc_price) = mid_prices.prices.get("BTC") {
    ///         println!("BTC mid price: ${}", btc_price.0);
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - Network request fails
    /// - API returns an error response
    /// - Response cannot be parsed
    pub async fn get_all_mids(&self) -> anyhow::Result<MidPrices> {
        self.make_basic_request("allMids").await
    }
}
