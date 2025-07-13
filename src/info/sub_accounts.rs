use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

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
   #[serde(with = "rust_decimal::serde::str")]
   pub cross_maintenance_margin_used: Decimal,
   #[serde(with = "rust_decimal::serde::str")]
   pub withdrawable: Decimal,
   #[serde(rename = "assetPositions")]
   pub asset_positions: Vec<serde_json::Value>,
   pub time: u64,
}

#[derive(Debug, Deserialize)]
pub struct MarginSummary {
   #[serde(rename = "accountValue")]
   #[serde(with = "rust_decimal::serde::str")]
   pub account_value: Decimal,
   #[serde(rename = "totalNtlPos")]
   #[serde(with = "rust_decimal::serde::str")]
   pub total_ntl_pos: Decimal,
   #[serde(rename = "totalRawUsd")]
   #[serde(with = "rust_decimal::serde::str")]
   pub total_raw_usd: Decimal,
   #[serde(rename = "totalMarginUsed")]
   #[serde(with = "rust_decimal::serde::str")]
   pub total_margin_used: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct SpotState {
   pub balances: Vec<Balance>,
}

#[derive(Debug, Deserialize)]
pub struct Balance {
   pub coin: String,
   pub token: u32,
   #[serde(with = "rust_decimal::serde::str")]
   pub total: Decimal,
   #[serde(with = "rust_decimal::serde::str")]
   pub hold: Decimal,
   #[serde(rename = "entryNtl")]
   #[serde(with = "rust_decimal::serde::str")]
   pub entry_ntl: Decimal,
}

impl HyperLiquidClient {
    pub async fn get_sub_accounts(&self, user: &str) -> anyhow::Result<SubAccountsResponse> {
        validate_ethereum_address(user)?;
        self.make_user_request("subAccounts", user).await
    }
}