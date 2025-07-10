use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct DelegationsRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

pub type DelegationsResponse = Vec<ValidatorStaking>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorStaking {
    pub validator: String,
    pub amount: String,
    pub locked_until_timestamp: u64,
}

impl HyperLiquidClient {
    pub async fn get_delegations(&self, user: &str) -> anyhow::Result<DelegationsResponse> {
        let url = format!("{}/info", self.base_url);
    
            let request_body = DelegationsRequest {
            request_type: "delegations".to_string(),
            user: user.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let max_builder_fee: DelegationsResponse = response.json().await?;
        Ok(max_builder_fee)
    }
}