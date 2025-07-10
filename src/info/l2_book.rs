use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct L2BookRequest {
    #[serde(rename = "type")]
    request_type: String,
    coin: String,
    #[serde(rename = "nSigFigs")]
    n_sig_figs: Option<u64>,
    #[serde(rename = "mantissa")]
    mantissa: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct L2BookResponse {
    pub coin: String,
    pub time: u64,
    pub levels: Vec<Vec<Level>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub px: String,
    pub sz: String,
    pub n: u32,
}


impl HyperLiquidClient {
    pub async fn get_l2_book(&self, coin: &str) -> anyhow::Result<L2BookResponse> {
        let url = format!("{}/info", self.base_url);
    
        let request_body = L2BookRequest {
            request_type: "l2Book".to_string(),
            coin: coin.to_string(),
            n_sig_figs: None,
            mantissa: None,
        };

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let l2_book: L2BookResponse = response.json().await?;
        Ok(l2_book)
    }
}