use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;


#[derive(Serialize)]
struct VaultDetailsRequest {
    #[serde(rename = "type")]
    request_type: String,
    #[serde(rename = "vaultAddress")]
    vault_address: String,
}

pub type VaultDetailsResponse = Option<VaultDetails>;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultDetails {
    pub name: String,
    pub vault_address: String,
    pub leader: String,
    pub description: String,
    pub portfolio: Vec<(String, PortfolioData)>,
    pub apr: f64,
    pub follower_state: Option<serde_json::Value>,
    pub leader_fraction: f64,
    pub leader_commission: f64,
    pub followers: Vec<Follower>,
    pub max_distributable: f64,
    pub max_withdrawable: f64,
    pub is_closed: bool,
    pub relationship: Relationship,
    pub allow_deposits: bool,
    pub always_close_on_withdraw: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioData {
    pub account_value_history: Vec<(u64, String)>,
    pub pnl_history: Vec<(u64, String)>,
    pub vlm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Follower {
    pub user: String,
    pub vault_equity: String,
    pub pnl: String,
    pub all_time_pnl: String,
    pub days_following: u32,
    pub vault_entry_time: u64,
    pub lockup_until: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    #[serde(rename = "type")]
    pub relationship_type: String,
}

impl HyperLiquidClient {
    pub async fn get_vault_details(&self, vault_address: &str) -> anyhow::Result<VaultDetailsResponse> {
        let url = format!("{}/info", self.base_url);

        let request_body = VaultDetailsRequest {
            request_type: "vaultDetails".to_string(),
            vault_address: vault_address.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let vault_details: VaultDetailsResponse = response.json().await?;
        Ok(vault_details)
    }
}   