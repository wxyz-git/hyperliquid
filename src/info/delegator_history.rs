use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

pub type DelegatorHistoryResponse = Vec<DelegatorHistory>;

#[derive(Deserialize, Debug)]
pub struct DelegatorHistory {
    pub time: u64,
    pub hash: String,
    pub delta: DelegatorHistoryDelta,
}

#[derive(Deserialize, Debug)]
pub struct DelegatorHistoryDelta {
    pub delegate: Option<DelegatorHistoryDelegate>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DelegatorHistoryDelegate {
    pub validator: String,
    pub amount: String,
    pub is_undelegate: bool,
}

impl HyperLiquidClient {
    pub async fn get_delegator_history(&self, user: &str) -> anyhow::Result<DelegatorHistoryResponse> {
        validate_ethereum_address(user)?;
        self.make_user_request("delegatorHistory", user).await
    }
}