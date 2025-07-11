use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;

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
        validate_ethereum_address(user)?;
        
        let request_body = MaxBuilderFeeRequest {
            request_type: "maxBuilderFee".to_string(),
            user: user.to_string(),
            builder: builder.to_string(),
        };

        self.make_custom_request(&request_body).await
    }
}