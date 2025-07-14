use rust_decimal::Decimal;
use serde::Deserialize;

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRateLimitResponse {
    #[serde(with = "rust_decimal::serde::str")]
    pub cum_vlm: Decimal,
    pub n_requests_used: u64,
    pub n_requests_cap: u64,
}

impl HyperLiquidClient {
    pub async fn get_user_rate_limit(&self, user: &str) -> anyhow::Result<UserRateLimitResponse> {
        validate_ethereum_address(user)?;
        self.make_user_request("userRateLimit", user).await
    }
}