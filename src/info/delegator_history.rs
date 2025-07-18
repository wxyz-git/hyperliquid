use rust_decimal::Decimal;
use serde::Deserialize;

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

pub type DelegatorHistoryResponse = Vec<DelegatorHistory>;

#[derive(Debug, Deserialize)]
pub struct DelegatorHistory {
    pub time: u64,
    pub hash: String,
    pub delta: DelegatorHistoryDelta,
}

#[derive(Debug, Deserialize)]
pub struct DelegatorHistoryDelta {
    pub delegate: Option<DelegatorHistoryDelegate>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegatorHistoryDelegate {
    pub validator: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    pub is_undelegate: bool,
}

impl HyperLiquidClient {
    pub async fn get_delegator_history(&self, user: &str) -> anyhow::Result<DelegatorHistoryResponse> {
        validate_ethereum_address(user)?;
        self.make_user_request("delegatorHistory", user).await
    }
}