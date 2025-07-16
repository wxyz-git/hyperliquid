use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;
use crate::errors::validate_coin_symbol;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct L2BookRequest {
    #[serde(rename = "type")]
    request_type: String,
    coin: String,
    n_sig_figs: Option<u64>,
    mantissa: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct L2BookResponse {
    pub coin: String,
    pub time: u64,
    pub levels: Vec<Vec<Level>>,
}

#[derive(Debug, Deserialize)]
pub struct Level {
    /// Price level
    #[serde(with = "rust_decimal::serde::str")]
    pub px: Decimal,
    /// Size/quantity at this price level
    #[serde(with = "rust_decimal::serde::str")]
    pub sz: Decimal,
    /// Number of orders at this level
    pub n: u32,
}

impl HyperLiquidClient {
    pub async fn get_l2_book(&self, coin: &str) -> anyhow::Result<L2BookResponse> {
        validate_coin_symbol(coin)?;
        
        let request_body = L2BookRequest {
            request_type: "l2Book".to_string(),
            coin: coin.to_string(),
            n_sig_figs: None,
            mantissa: None,
        };

        self.make_custom_request(&request_body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_get_l2_book() {
        let client = HyperLiquidClient::new();
        let result = client.get_l2_book("BTC").await.unwrap();
        
        assert_eq!(result.coin, "BTC");
        assert!(result.time > 0);
        assert_eq!(result.levels.len(), 2);
        println!("✅ L2 book test passed - {} levels", result.levels.iter().map(|l| l.len()).sum::<usize>());
    }
}