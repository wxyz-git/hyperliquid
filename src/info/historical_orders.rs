use serde::Deserialize;

use crate::{client::HyperLiquidClient, errors::validate_ethereum_address, types::Order};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalOrdersResponse {
    pub order: Order,
    pub status: OrderStatus,
    pub status_timestamp: u64,
}

#[derive(Debug, Deserialize)]
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
        validate_ethereum_address(user)?;
        self.make_user_request("historicalOrders", user).await
    }
}