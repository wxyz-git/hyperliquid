use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

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
        validate_ethereum_address(user)?;
        
        let request_body = OrderStatusRequest {
            request_type: "orderStatus".to_string(),
            user: user.to_string(),
            oid: oid,
        };
    
        self.make_custom_request(&request_body).await
    }   
}



