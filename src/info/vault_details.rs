use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

#[derive(Serialize)]
struct VaultDetailsRequest {
    #[serde(rename = "type")]
    request_type: String,
    #[serde(rename = "vaultAddress")]
    vault_address: String,
}

pub type VaultDetailsResponse = Option<VaultDetails>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultDetails {
    pub name: String,
    pub vault_address: String,
    pub leader: String,
    pub description: String,
    pub portfolio: Vec<(String, PortfolioData)>,
    #[serde(with = "rust_decimal::serde::str")]
    pub apr: Decimal,
    pub follower_state: Option<serde_json::Value>,
    #[serde(with = "rust_decimal::serde::str")]
    pub leader_fraction: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub leader_commission: Decimal,
    pub followers: Vec<Follower>,
    #[serde(with = "rust_decimal::serde::str")]
    pub max_distributable: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub max_withdrawable: Decimal,
    pub is_closed: bool,
    pub relationship: Relationship,
    pub allow_deposits: bool,
    pub always_close_on_withdraw: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioData {
    pub account_value_history: Vec<HistoryEntry>,
    pub pnl_history: Vec<HistoryEntry>,
    #[serde(with = "rust_decimal::serde::str")]
    pub vlm: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct HistoryEntry(pub u64, #[serde(with = "rust_decimal::serde::str")] pub Decimal);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Follower {
    pub user: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub vault_equity: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub pnl: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub all_time_pnl: Decimal,
    pub days_following: u32,
    pub vault_entry_time: u64,
    pub lockup_until: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    #[serde(rename = "type")]
    pub relationship_type: String,
}

impl HyperLiquidClient {
    pub async fn get_vault_details(&self, vault_address: &str) -> anyhow::Result<VaultDetailsResponse> {
        validate_ethereum_address(vault_address)?;
        
        let request_body = VaultDetailsRequest {
            request_type: "vaultDetails".to_string(),
            vault_address: vault_address.to_string(),
        };

        self.make_custom_request(&request_body).await
    }
}   