use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct UserVaultEquitiesRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

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
        let url = format!("{}/info", self.base_url);


        let request_body = UserVaultEquitiesRequest {
            request_type: "userVaultEquities".to_string(),
            user: user.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let user_vault_equities: UserVaultEquitiesResponse = response.json().await?;
    Ok(user_vault_equities)
    }     
}