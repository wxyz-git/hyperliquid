use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct DelegatorRewardsRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

pub type DelegatorRewardsResponse = Vec<DelegatorRewards>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DelegatorRewards {
    pub time: u64,
    pub source: String,
    pub total_amount: String,
}


impl HyperLiquidClient {
    pub async fn get_delegator_rewards(&self, user: &str) -> anyhow::Result<DelegatorRewardsResponse> {
        let url = format!("{}/info", self.base_url);

        let request_body = DelegatorRewardsRequest {
            request_type: "delegatorRewards".to_string(),
            user: user.to_string(),
        };

        let response = self.client.post(&url).header("Content-Type", "application/json").json(&request_body).send().await?;
        let delegator_rewards: DelegatorRewardsResponse = response.json().await?;
        Ok(delegator_rewards)
    }
}   