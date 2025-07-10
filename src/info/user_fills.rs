use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct UserFillsRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

#[derive(Deserialize, Debug)]
pub struct UserFillsResponse {
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
    pub cloid: Option<String>,
    #[serde(rename = "feeToken")]
    pub fee_token: String,
}

impl HyperLiquidClient {
    pub async fn get_user_fills(&self, user: &str) -> anyhow::Result<Vec<UserFillsResponse>> {
        let url = format!("{}/info", self.base_url);
        
        let request_body = UserFillsRequest {
            request_type: "userFills".to_string(),
            user: user.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let user_fills: Vec<UserFillsResponse> = response.json().await?;
        Ok(user_fills)
    }
}