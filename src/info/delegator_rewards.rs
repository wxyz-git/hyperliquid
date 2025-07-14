use rust_decimal::Decimal;
use serde::Deserialize;

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

pub type DelegatorRewardsResponse = Vec<DelegatorRewards>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegatorRewards {
    pub time: u64,
    pub source: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub total_amount: Decimal,
}

impl HyperLiquidClient {
    pub async fn get_delegator_rewards(&self, user: &str) -> anyhow::Result<DelegatorRewardsResponse> {
        validate_ethereum_address(user)?;
        self.make_user_request("delegatorRewards", user).await
    }
}   