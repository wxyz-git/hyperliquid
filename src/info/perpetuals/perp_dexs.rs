use serde::Deserialize;
use crate::client::HyperLiquidClient;

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
        self.make_basic_request("perpDexs").await
    }
}