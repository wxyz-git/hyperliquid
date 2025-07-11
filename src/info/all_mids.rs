use std::collections::HashMap;
use serde::Deserialize;
use crate::client::HyperLiquidClient;

#[derive(Deserialize, Debug)]
pub struct MidPrices(pub HashMap<String, String>);

impl HyperLiquidClient {
    pub async fn get_all_mids(&self) -> anyhow::Result<MidPrices> {
        self.make_basic_request("allMids").await
    }
}
