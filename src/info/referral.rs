use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct ReferralRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferralResponse {
    pub referred_by: Option<String>,
    pub cum_vlm: String,
    pub unclaimed_rewards: String,
    pub claimed_rewards: String,
    pub builder_rewards: String,
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
    pub amount: String,
}

impl HyperLiquidClient {
    pub async fn get_referral_info(&self, user: &str) -> anyhow::Result<ReferralResponse> {
        let url = format!("{}/info", self.base_url);

        let request_body = ReferralRequest {
            request_type: "referral".to_string(),
            user: user.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let sub_accounts: ReferralResponse = response.json().await?;
        Ok(sub_accounts)
    }
}