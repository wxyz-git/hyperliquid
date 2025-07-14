use rust_decimal::Decimal;
use serde::Deserialize;

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrdersResponse {
    pub coin: String,
    pub side: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub limit_px: Decimal,
    pub oid: u64,
    pub timestamp: u64,
    #[serde(with = "rust_decimal::serde::str")]
    pub orig_sz: Decimal,
    pub cloid: String,
}

impl HyperLiquidClient {
    pub async fn get_open_orders(&self, user: &str) -> anyhow::Result<Vec<OpenOrdersResponse>> {
        validate_ethereum_address(user)?;
        self.make_user_request("openOrders", user).await
    }
}