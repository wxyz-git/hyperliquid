use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct DelegatorHistoryRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

pub type DelegatorHistoryResponse = Vec<DelegatorHistory>;

#[derive(Deserialize, Debug)]
pub struct DelegatorHistory {
    pub time: u64,
    pub hash: String,
    pub delta: DelegatorHistoryDelta,
}

#[derive(Deserialize, Debug)]
pub struct DelegatorHistoryDelta {
    pub delegate: Option<DelegatorHistoryDelegate>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DelegatorHistoryDelegate {
    pub validator: String,
    pub amount: String,
    pub is_undelegate: bool,
}


impl HyperLiquidClient {
    pub async fn get_delegator_history(&self, user: &str) -> anyhow::Result<DelegatorHistoryResponse> {
        let url = format!("{}/info", self.base_url);

        let request_body = DelegatorHistoryRequest {
            request_type: "delegatorHistory".to_string(),
            user: user.to_string(),
        };

        let response = self.client.post(&url).header("Content-Type", "application/json").json(&request_body).send().await?;
        let delegator_history: DelegatorHistoryResponse = response.json().await?;
        Ok(delegator_history)
    }
}