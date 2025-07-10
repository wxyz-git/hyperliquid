use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;


#[derive(Serialize)]
struct SubAccountsRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

pub type SubAccountsResponse = Option<Vec<SubAccounts>>;

#[derive(Debug, Deserialize)]
pub struct SubAccounts {
   pub name: String,
   #[serde(rename = "subAccountUser")]
   pub sub_account_user: String,
   pub master: String,
   #[serde(rename = "clearinghouseState")]
   pub clearinghouse_state: ClearinghouseState,
   #[serde(rename = "spotState")]
   pub spot_state: SpotState,
}

#[derive(Debug, Deserialize)]
pub struct ClearinghouseState {
   #[serde(rename = "marginSummary")]
   pub margin_summary: MarginSummary,
   #[serde(rename = "crossMarginSummary")]
   pub cross_margin_summary: MarginSummary,
   #[serde(rename = "crossMaintenanceMarginUsed")]
   pub cross_maintenance_margin_used: String,
   pub withdrawable: String,
   #[serde(rename = "assetPositions")]
   pub asset_positions: Vec<serde_json::Value>,
   pub time: u64,
}

#[derive(Debug, Deserialize)]
pub struct MarginSummary {
   #[serde(rename = "accountValue")]
   pub account_value: String,
   #[serde(rename = "totalNtlPos")]
   pub total_ntl_pos: String,
   #[serde(rename = "totalRawUsd")]
   pub total_raw_usd: String,
   #[serde(rename = "totalMarginUsed")]
   pub total_margin_used: String,
}

#[derive(Debug, Deserialize)]
pub struct SpotState {
   pub balances: Vec<Balance>,
}

#[derive(Debug, Deserialize)]
pub struct Balance {
   pub coin: String,
   pub token: u32,
   pub total: String,
   pub hold: String,
   #[serde(rename = "entryNtl")]
   pub entry_ntl: String,
}

impl HyperLiquidClient {
    pub async fn get_sub_accounts(&self, user: &str) -> anyhow::Result<SubAccountsResponse> {
        let url = format!("{}/info", self.base_url);

        let request_body = SubAccountsRequest {
            request_type: "subAccounts".to_string(),
            user: user.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let sub_accounts: SubAccountsResponse = response.json().await?;
        Ok(sub_accounts)
    }
}