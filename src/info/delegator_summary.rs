use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct DelegatorSummaryRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DelegatorSummaryResponse {
    pub delegated: String,
    pub undelegated: String,
    pub total_pending_withdrawal: String,
    pub n_pending_withdrawals: u64,
}

impl HyperLiquidClient {
    pub async fn get_delegator_summary(&self, user: &str) -> anyhow::Result<DelegatorSummaryResponse> {
        let url = format!("{}/info", self.base_url);

        let request_body = DelegatorSummaryRequest {
            request_type: "delegatorSummary".to_string(),
            user: user.to_string(),
        };

        let response = self.client.post(&url).header("Content-Type", "application/json").json(&request_body).send().await?;
        let delegator_summary: DelegatorSummaryResponse = response.json().await?;
        Ok(delegator_summary)
    }
}
