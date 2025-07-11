use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;


#[derive(Serialize)]
struct OpenOrdersRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

#[derive(Deserialize, Debug)]
pub struct OpenOrdersResponse {
    pub coin: String,
    pub side: String,
    #[serde(rename = "limitPx")]
    pub limit_px: String,
    pub oid: u64,
    pub timestamp: u64,
    #[serde(rename = "origSz")]
    pub origsz: String,
    pub cloid: String,
}

impl HyperLiquidClient {
    pub async fn get_open_orders(&self, user: &str) -> anyhow::Result<Vec<OpenOrdersResponse>> {
        let url = format!("{}/info", self.base_url);
        
        let request_body = OpenOrdersRequest {
            request_type: "openOrders".to_string(),
            user: user.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let open_orders: Vec<OpenOrdersResponse> = response.json().await?;
        Ok(open_orders)
    }
}