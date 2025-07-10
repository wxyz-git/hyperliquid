use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct UserRateLimitRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

#[derive(Deserialize, Debug)]
pub struct UserRateLimitResponse {
    #[serde(rename = "cumVlm")]
    pub cum_vlm: String,
    #[serde(rename = "nRequestsUsed")]
    pub n_requests_used: u64,
    #[serde(rename = "nRequestsCap")]
    pub n_requests_cap: u64,
}

impl HyperLiquidClient {
    pub async fn get_user_rate_limit(&self, user: &str) -> anyhow::Result<UserRateLimitResponse> {
        let url = format!("{}/info", self.base_url);
    
        let request_body = UserRateLimitRequest {
            request_type: "userRateLimit".to_string(),
            user: user.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let user_rate_limit: UserRateLimitResponse = response.json().await?;
        Ok(user_rate_limit)
    }
}