use serde::Deserialize;

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

#[derive(Debug, Deserialize)]
pub struct UserRoleResponse {
   pub role: Role,
}

#[derive(Debug, Deserialize)]
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
        validate_ethereum_address(user)?;
        self.make_user_request("userRole", user).await
    }
}