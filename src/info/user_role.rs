use serde::{Serialize, Deserialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct UserRoleRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}

#[derive(Debug, Deserialize)]
pub struct UserRoleResponse {
    role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Missing,
    User,
    Agent,
    Vault,
    SubAccount,
}

impl HyperLiquidClient {
    pub async fn get_user_role(&self, user: &str) -> anyhow::Result<UserRoleResponse> {
        let url = format!("{}/info", self.base_url);

        let request_body = UserRoleRequest {
            request_type: "userRole".to_string(),
            user: user.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body) 
            .send()
            .await?;

        let user_role: UserRoleResponse = response.json().await?;
    Ok(user_role)
    }
}