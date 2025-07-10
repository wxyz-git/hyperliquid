use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize, Debug)]
struct PerpDexsRequest {
    #[serde(rename = "type")]
    request_type: String,
}

pub type PerpDexsResponse = Vec<Option<PerpDex>>;

#[derive(Deserialize, Debug)]
pub struct PerpDex {
    pub name: String,
    pub full_name: String,
    pub deployer: String,
    pub oracle_updater: String,
}

impl HyperLiquidClient {
    pub async fn get_perp_dexs(&self) -> anyhow::Result<PerpDexsResponse> {
        let url = format!("{}/info", self.base_url);

        let request_body = PerpDexsRequest {
            request_type: "perpDexs".to_string(),
        };

        let response = self.client.post(&url).header("Content-Type", "application/json").json(&request_body).send().await?;
        let perp_dexs: PerpDexsResponse = response.json().await?;
        Ok(perp_dexs)
    }
}