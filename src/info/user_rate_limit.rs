use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

#[derive(Deserialize, Debug)]
pub struct UserRateLimitResponse {
    #[serde(rename = "cumVlm")]
    #[serde(with = "rust_decimal::serde::str")]
    pub cum_vlm: Decimal,
    #[serde(rename = "nRequestsUsed")]
    pub n_requests_used: u64,
    #[serde(rename = "nRequestsCap")]
    pub n_requests_cap: u64,
}

impl HyperLiquidClient {
    pub async fn get_user_rate_limit(&self, user: &str) -> anyhow::Result<UserRateLimitResponse> {
        validate_ethereum_address(user)?;
        self.make_user_request("userRateLimit", user).await
    }
}