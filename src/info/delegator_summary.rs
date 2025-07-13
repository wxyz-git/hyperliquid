use rust_decimal::Decimal;
use serde::Deserialize;
use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DelegatorSummaryResponse {
    #[serde(with = "rust_decimal::serde::str")]
    pub delegated: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub undelegated: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub total_pending_withdrawal: Decimal,
    pub n_pending_withdrawals: u64,
}

impl HyperLiquidClient {
    pub async fn get_delegator_summary(&self, user: &str) -> anyhow::Result<DelegatorSummaryResponse> {
        validate_ethereum_address(user)?;
        self.make_user_request("delegatorSummary", user).await
    }
}
