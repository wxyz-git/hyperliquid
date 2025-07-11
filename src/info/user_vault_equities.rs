use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

pub type UserVaultEquitiesResponse = Option<Vec<VaultPosition>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultPosition {
    pub vault_address: String,
    pub equity: String,
    pub locked_until_timestamp: u64,
}

impl HyperLiquidClient {
    pub async fn get_user_vault_equities(&self, user: &str) -> anyhow::Result<UserVaultEquitiesResponse> {
        validate_ethereum_address(user)?;
        self.make_user_request("userVaultEquities", user).await
    }
}