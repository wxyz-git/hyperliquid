use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Deserialize, Serialize)]
struct AllMidsRequest {
    #[serde(rename = "type")]
    request_type: String,
}

#[derive(Deserialize, Debug)]
pub struct MidPrices(pub HashMap<String, String>);

impl HyperLiquidClient {
    pub async fn get_all_mids(&self) -> Result<MidPrices, Box<dyn std::error::Error>> {
        let url = format!("{}/info", self.base_url);
        
        let request_body = AllMidsRequest {
            request_type: "allMids".to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let mid_prices: MidPrices = response.json().await?;
        Ok(mid_prices)
    }
}
