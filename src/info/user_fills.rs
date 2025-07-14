use rust_decimal::Decimal;
use serde::Deserialize;

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFillsResponse {
    pub coin: String,
    /// Fill price
    #[serde(with = "rust_decimal::serde::str")]
    pub px: Decimal,
    /// Fill size
    #[serde(with = "rust_decimal::serde::str")]
    pub sz: Decimal,
    pub side: String,
    pub time: u64,
    /// Starting position before this fill
    #[serde(with = "rust_decimal::serde::str")]
    pub start_position: Decimal,
    pub dir: String,
    /// Closed PnL from this fill
    #[serde(with = "rust_decimal::serde::str")]
    pub closed_pnl: Decimal,
    pub hash: String,
    pub oid: u64,
    pub crossed: bool,
    /// Fee paid for this fill
    #[serde(with = "rust_decimal::serde::str")]
    pub fee: Decimal,
    pub tid: u64,
    pub cloid: Option<String>,
    pub fee_token: String,
}

impl HyperLiquidClient {
    pub async fn get_user_fills(&self, user: &str) -> anyhow::Result<Vec<UserFillsResponse>> {
        validate_ethereum_address(user)?;
        self.make_user_request("userFills", user).await
    }
}