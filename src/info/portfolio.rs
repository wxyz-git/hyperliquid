use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

pub type PortfolioResponse = Vec<PortfolioHistoryEntry>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioHistoryEntry(pub String, pub PortfolioHistoryData);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioHistoryData {
    pub account_value_history: Vec<(u64, String)>,
    pub pnl_history: Vec<(u64, String)>,
    pub vlm: String,
}

impl HyperLiquidClient {
    pub async fn get_portfolio(&self, user: &str) -> anyhow::Result<PortfolioResponse> {
        validate_ethereum_address(user)?;
        self.make_user_request("portfolio", user).await
    }
}