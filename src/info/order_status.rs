use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct OrderStatusRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
    oid: u64,
}

#[derive(Deserialize, Debug)]
pub struct OrderStatusResponse {
    pub status: String,
    pub order: OrderStatus,
}

#[derive(Deserialize, Debug)]
pub struct OrderStatus {
    pub order: Order,
    pub status: String,
    #[serde(rename = "statusTimestamp")]
    pub status_timestamp: u64,
}

#[derive(Deserialize, Debug)]
pub struct Order{
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

    pub async fn get_order_status(&self, user: &str, oid: u64) -> anyhow::Result<OrderStatusResponse> {
        let url = format!("{}/info", self.base_url);
        
        let request_body = OrderStatusRequest {
            request_type: "orderStatus".to_string(),
            user: user.to_string(),
            oid: oid,
        };
    
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;
        
        let order_status: OrderStatusResponse = response.json().await?;
        Ok(order_status)
    }   
}



