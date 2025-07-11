use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct MaxBuilderFeeRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
    builder: String,
}

#[derive(Deserialize, Debug)]
pub struct MaxBuilderFeeResponse {
    pub fee: f64,
}

impl HyperLiquidClient {
    pub async fn get_max_builder_fee(&self, user: &str, builder: &str) -> anyhow::Result<MaxBuilderFeeResponse> {
        let url = format!("{}/info", self.base_url);
    
        let request_body = MaxBuilderFeeRequest {
            request_type: "maxBuilderFee".to_string(),
            user: user.to_string(),
            builder: builder.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let max_builder_fee: MaxBuilderFeeResponse = response.json().await?;
        Ok(max_builder_fee)
    }
}