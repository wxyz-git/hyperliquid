use reqwest;
pub struct HyperLiquidClient {
    pub client: reqwest::Client,
    pub base_url: String,
}

impl HyperLiquidClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.hyperliquid.xyz".to_string(),
        }
    }
}  