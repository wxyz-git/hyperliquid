use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;
use crate::errors::{validate_coin_symbol, validate_time_range, validate_interval};

#[derive(Serialize)]
struct CandleSnapshotRequest {
    #[serde(rename = "type")]
    request_type: String,
    req: CandleRequest,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CandleRequest {
    coin: String,
    interval: String,
    start_time: u64,
    end_time: u64,
}

#[derive(Debug, Deserialize)]
pub struct CandleData {
    /// Start time of the candle in epoch milliseconds
    pub t: u64,
    /// End time of the candle in epoch milliseconds
    #[serde(rename = "T")]
    pub end_time: u64,
    /// Symbol/coin name
    pub s: String,
    /// Interval
    pub i: String,
    /// Open price
    #[serde(with = "rust_decimal::serde::str")]
    pub o: Decimal,
    /// Close price
    #[serde(with = "rust_decimal::serde::str")]
    pub c: Decimal,
    /// High price
    #[serde(with = "rust_decimal::serde::str")]
    pub h: Decimal,
    /// Low price
    #[serde(with = "rust_decimal::serde::str")]
    pub l: Decimal,
    /// Volume
    #[serde(with = "rust_decimal::serde::str")]
    pub v: Decimal,
    /// Number of trades
    pub n: u32,
}

impl HyperLiquidClient {
    pub async fn get_candle_snapshot(&self, coin: &str, interval: &str, start_time: u64, end_time: u64) -> anyhow::Result<Vec<CandleData>> {
        // Validate inputs
        validate_coin_symbol(coin)?;
        validate_interval(interval)?;
        validate_time_range(start_time, end_time)?;
        
        let request_body = CandleSnapshotRequest {
            request_type: "candleSnapshot".to_string(),
            req: CandleRequest {
                coin: coin.to_string(),
                interval: interval.to_string(),
                start_time,
                end_time,
            },
        };

        self.make_custom_request(&request_body).await
    }
}