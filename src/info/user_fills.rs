use serde::Deserialize;

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

#[derive(Deserialize, Debug)]
pub struct UserFillsResponse {
    pub coin: String,
    pub px: String,
    pub sz: String,
    pub side: String,
    pub time: u64,
    #[serde(rename = "startPosition")]
    pub start_position: String,
    pub dir: String,
    #[serde(rename = "closedPnl")]
    pub closed_pnl: String,
    pub hash: String,
    pub oid: u64,
    pub crossed: bool,
    pub fee: String,
    pub tid: u64,
    pub cloid: Option<String>,
    #[serde(rename = "feeToken")]
    pub fee_token: String,
}

impl HyperLiquidClient {
    pub async fn get_user_fills(&self, user: &str) -> anyhow::Result<Vec<UserFillsResponse>> {
        validate_ethereum_address(user)?;
        self.make_user_request("userFills", user).await
    }
}