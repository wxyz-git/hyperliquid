use serde::Deserialize;

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

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
        validate_ethereum_address(user)?;
        self.make_user_request("openOrders", user).await
    }
}