use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct CandleSnapshotRequest {
    #[serde(rename = "type")]
    request_type: String,
    req: CandleRequest,
}

#[derive(Serialize)]
struct CandleRequest {
    coin: String,
    interval: String,
    #[serde(rename = "startTime")]
    start_time: u64,
    #[serde(rename = "endTime")]
    end_time: u64,
}

#[derive(Deserialize, Debug)]
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
    pub o: String,
    /// Close price
    pub c: String,
    /// High price
    pub h: String,
    /// Low price
    pub l: String,
    /// Volume
    pub v: String,
    /// Number of trades
    pub n: u32,
}

impl HyperLiquidClient {
    pub async fn get_candle_snapshot(&self, coin: &str, interval: &str, start_time: u64, end_time: u64) -> anyhow::Result<Vec<CandleData>> {
        let url = format!("{}/info", self.base_url);
    
        let request_body = CandleSnapshotRequest {
            request_type: "candleSnapshot".to_string(),
            req: CandleRequest {
                coin: coin.to_string(),
                interval: interval.to_string(),
                start_time,
                end_time,
            },
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let candle_data: Vec<CandleData> = response.json().await?;
        Ok(candle_data)
    }
}