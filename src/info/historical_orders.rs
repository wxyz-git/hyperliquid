use serde::Deserialize;

use crate::{client::HyperLiquidClient, errors::validate_ethereum_address, types::{Order, OrderState}};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalOrdersResponse {
    pub order: Order,
    pub status: OrderState,
    pub status_timestamp: u64,
}

impl HyperLiquidClient {
    pub async fn get_historical_orders(&self, user: &str) -> anyhow::Result<Vec<HistoricalOrdersResponse>> {
        validate_ethereum_address(user)?;
        self.make_user_request("historicalOrders", user).await
    }
}