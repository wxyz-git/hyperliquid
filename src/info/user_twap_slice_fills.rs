use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct UserTwapSliceFillsRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTwapSliceFillsResponse {
    pub fill: Fill,
    #[serde(rename = "twapId")]
    pub twap_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill {
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
}


impl HyperLiquidClient {
    pub async fn get_user_twap_slice_fills(&self, user: &str) -> anyhow::Result<Vec<UserTwapSliceFillsResponse>> {
        let url = format!("{}/info", self.base_url);
    
        let request_body = UserTwapSliceFillsRequest {
            request_type: "userTwapSliceFills".to_string(),
            user: user.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let user_twap_slice_fills: Vec<UserTwapSliceFillsResponse> = response.json().await?;
        Ok(user_twap_slice_fills)
    }
}