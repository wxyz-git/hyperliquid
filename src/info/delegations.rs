use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

pub type DelegationsResponse = Vec<ValidatorStaking>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorStaking {
    pub validator: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    pub locked_until_timestamp: u64,
}

impl HyperLiquidClient {
    pub async fn get_delegations(&self, user: &str) -> anyhow::Result<DelegationsResponse> {
        validate_ethereum_address(user)?;
        self.make_user_request("delegations", user).await
    }
}