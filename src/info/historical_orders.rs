use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct HistoricalOrdersRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HistoricalOrdersResponse {
    pub order: Order,
    pub status: OrderStatus,
    #[serde(rename = "statusTimestamp")]
    pub status_timestamp: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Order {
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
    pub children: Vec<serde_json::Value>, // Generic for now, can be more specific if needed
    #[serde(rename = "isPositionTpsl")]
    pub is_position_tpsl: bool,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "orderType")]
    pub order_type: String,
    #[serde(rename = "origSz")]
    pub orig_sz: String,
    pub tif: String,
    pub cloid: Option<String>,
}


#[derive(Deserialize, Debug, Clone)]
pub enum OrderStatus {
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "triggered")]
    Triggered,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "marginCanceled")]
    MarginCanceled,
    #[serde(rename = "reduceOnlyRejected")]
    ReduceOnlyRejected,
    // Add other variants as they are discovered
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Filled => write!(f, "filled"),
            OrderStatus::Open => write!(f, "open"),
            OrderStatus::Canceled => write!(f, "canceled"),
            OrderStatus::Triggered => write!(f, "triggered"),
            OrderStatus::Rejected => write!(f, "rejected"),
            OrderStatus::MarginCanceled => write!(f, "marginCanceled"),
            OrderStatus::ReduceOnlyRejected => write!(f, "reduceOnlyRejected"),
            OrderStatus::Unknown => write!(f, "unknown"),
        }
    }
}

impl HyperLiquidClient {
    pub async fn get_historical_orders(&self, user: &str) -> anyhow::Result<Vec<HistoricalOrdersResponse>> {
        let url = format!("{}/info", self.base_url);
    
        let request_body = HistoricalOrdersRequest {
            request_type: "historicalOrders".to_string(),
            user: user.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let historical_orders: Vec<HistoricalOrdersResponse> = response.json().await?;
        Ok(historical_orders)
    }
}