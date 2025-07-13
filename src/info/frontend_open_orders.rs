use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FrontendOpenOrdersResponse {
    pub coin: String,
    pub side: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub limit_px: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub sz: Decimal,
    pub oid: u64,
    pub timestamp: u64,
    pub trigger_condition: String,
    pub is_trigger: bool,
    #[serde(with = "rust_decimal::serde::str")]
    pub trigger_px: Decimal,
    pub children: Vec<String>,
    pub is_position_tpsl: bool,
    pub reduce_only: bool,
    pub order_type: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub origsz: Decimal,
    pub tif: String,
    #[serde(default)]
    pub cloid: Option<String>,
}

impl HyperLiquidClient {
    pub async fn get_frontend_open_orders(&self, user: &str) -> anyhow::Result<Vec<FrontendOpenOrdersResponse>> {
        validate_ethereum_address(user)?;
        self.make_user_request("frontendOpenOrders", user).await
    }
}