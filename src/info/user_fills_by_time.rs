use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize, Debug)]
struct UserFillsByTimeRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
    #[serde(rename = "startTime")]
    start_time: u64,
    #[serde(rename = "endTime")]
    end_time: Option<u64>,
    #[serde(rename = "aggregatedByTime")]
    aggregated_by_time: bool,
}

#[derive(Deserialize, Debug)]
pub struct UserFillsByTimeResponse {
    pub coin: String,
    pub px: String,
    pub sz: String,
    pub side: String,
    pub time: u64,
    #[serde(rename = "startPosition")]
    pub start_position: String,
    pub dir: String,
    #[serde(rename = "closedPnl")]
    pub closed_pnl: String,
    pub hash: String,
    pub oid: u64,
    pub crossed: bool,
    pub fee: String,
    pub tid: u64,
    #[serde(rename = "feeToken")]
    pub fee_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidation: Option<Liquidation>,
}

#[derive(Deserialize, Debug)]
pub struct Liquidation {
    #[serde(rename = "liquidatedUser")]
    pub liquidated_user: String,
    #[serde(rename = "markPx")]
    pub mark_px: String,
    pub method: String,
}

impl HyperLiquidClient {
    pub async fn get_user_fills_by_time(&self, user: &str, start_time: u64, end_time: Option<u64>, aggregated_by_time: bool) -> anyhow::Result<Vec<UserFillsByTimeResponse>> {
        let url = format!("{}/info", self.base_url);
    
        let request_body = UserFillsByTimeRequest {
            request_type: "userFillsByTime".to_string(),
            user: user.to_string(),
            start_time: start_time,
            end_time: end_time,
            aggregated_by_time: aggregated_by_time,
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let user_fills: Vec<UserFillsByTimeResponse> = response.json().await?;
        Ok(user_fills)
    }
}