use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct UserFeesRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFeesResponse {
    pub daily_user_vlm: Vec<DailyUserVlm>,
    pub fee_schedule: FeeSchedule,
    pub user_cross_rate: String,
    pub user_add_rate: String,
    pub user_spot_cross_rate: String,
    pub user_spot_add_rate: String,
    pub active_referral_discount: String,
    pub trial: Option<String>,
    pub fee_trial_reward: String,
    pub next_trial_available_timestamp: Option<u64>,
    pub staking_link: Option<String>,
    pub active_staking_discount: Option<ActiveStakingDiscount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyUserVlm {
    pub date: String,
    pub user_cross: String,
    pub user_add: String,
    pub exchange: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeSchedule {
    pub cross: String,
    pub add: String,
    pub spot_cross: String,
    pub spot_add: String,
    pub tiers: Tiers,
    pub referral_discount: String,
    pub staking_discount_tiers: Vec<StakingDiscountTier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tiers {
    pub vip: Vec<VipTier>,
    pub mm: Vec<MmTier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VipTier {
    pub ntl_cutoff: String,
    pub cross: String,
    pub add: String,
    pub spot_cross: String,
    pub spot_add: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmTier {
    pub maker_fraction_cutoff: String,
    pub add: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakingDiscountTier {
    pub bps_of_max_supply: String,
    pub discount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveStakingDiscount {
    pub bps_of_max_supply: String,
    pub discount: String,
}

impl HyperLiquidClient {
    pub async fn get_user_fees(&self, user: &str) -> anyhow::Result<UserFeesResponse> {
        let url = format!("{}/info", self.base_url);

        let request_body = UserFeesRequest {
            request_type: "userFees".to_string(),
            user: user.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let user_fees: UserFeesResponse = response.json().await?;
        Ok(user_fees)
    }
}