use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralResponse {
    pub referred_by: Option<String>,
    #[serde(with = "rust_decimal::serde::str")]
    pub cum_vlm: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub unclaimed_rewards: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub claimed_rewards: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub builder_rewards: Decimal,
    pub referrer_state: ReferrerState,
    pub reward_history: Vec<RewardHistoryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferrerState {
    pub stage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardHistoryItem {
    pub time: u64,
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
}

impl HyperLiquidClient {
    pub async fn get_referral_info(&self, user: &str) -> anyhow::Result<ReferralResponse> {
        validate_ethereum_address(user)?;
        self.make_user_request("referral", user).await
    }
}