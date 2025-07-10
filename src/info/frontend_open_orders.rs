use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;


#[derive(Serialize)]
struct FrontendOpenOrdersRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

#[derive(Deserialize, Debug)]
pub struct FrontendOpenOrdersResponse {
    pub coin: String,
    pub side: String,
    #[serde(rename = "limitPx")]
    pub limit_px: String,
    pub sz: String,
    pub oid: u64,
    pub timestamp: u64,
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: String,
    #[serde(rename = "isTrigger")]
    pub is_trigger: bool,
    #[serde(rename = "triggerPx")]
    pub trigger_px: String,
    pub children: Vec<String>,
    #[serde(rename = "isPositionTpsl")]
    pub is_position_tpsl: bool,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "origSz")]
    pub origsz: String,
    pub tif: String,
    #[serde(default)]
    pub cloid: Option<String>,
}

impl HyperLiquidClient {
    pub async fn get_frontend_open_orders(&self, user: &str) -> anyhow::Result<Vec<FrontendOpenOrdersResponse>> {
        let url = format!("{}/info", self.base_url);
        
        let request_body = FrontendOpenOrdersRequest {
            request_type: "frontendOpenOrders".to_string(),
            user: user.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let frontend_open_orders: Vec<FrontendOpenOrdersResponse> = response.json().await?;
        Ok(frontend_open_orders)
    }
}