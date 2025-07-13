use serde::{Deserialize, Serialize};
use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTwapSliceFillsResponse {
    pub fill: Fill,
    #[serde(rename = "twapId")]
    pub twap_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill {
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
    #[serde(rename = "feeToken")]
    pub fee_token: String,
}

impl HyperLiquidClient {
    pub async fn get_user_twap_slice_fills(&self, user: &str) -> anyhow::Result<Vec<UserTwapSliceFillsResponse>> {
        validate_ethereum_address(user)?;
        self.make_user_request("userTwapSliceFills", user).await
    }
}