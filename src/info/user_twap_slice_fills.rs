use rust_decimal::Decimal;
use serde::Deserialize;

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTwapSliceFillsResponse {
    pub fill: Fill,
    pub twap_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    pub coin: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub px: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub sz: Decimal,
    pub side: String,
    pub time: u64,
    #[serde(with = "rust_decimal::serde::str")]
    pub start_position: Decimal,
    pub dir: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub closed_pnl: Decimal,
    pub hash: String,
    pub oid: u64,
    pub crossed: bool,
    #[serde(with = "rust_decimal::serde::str")]
    pub fee: Decimal,
    pub tid: u64,
    pub fee_token: String,
}

impl HyperLiquidClient {
    pub async fn get_user_twap_slice_fills(&self, user: &str) -> anyhow::Result<Vec<UserTwapSliceFillsResponse>> {
        validate_ethereum_address(user)?;
        self.make_user_request("userTwapSliceFills", user).await
    }
}