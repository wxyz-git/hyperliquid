use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct PortfolioRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

pub type PortfolioResponse = Vec<PortfolioHistoryEntry>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioHistoryEntry(pub String, pub PortfolioHistoryData);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioHistoryData {
    pub account_value_history: Vec<(u64, String)>,
    pub pnl_history: Vec<(u64, String)>,
    pub vlm: String,
}

impl HyperLiquidClient {
    pub async fn get_portfolio(&self, user: &str) -> anyhow::Result<PortfolioResponse> {
        let url = format!("{}/info", self.base_url);

        let request_body = PortfolioRequest {
            request_type: "portfolio".to_string(),
            user: user.to_string(),
        };

        let response = self
            .client 
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let portfolio: PortfolioResponse = response.json().await?;
        Ok(portfolio)
    }
}