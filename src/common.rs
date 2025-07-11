use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::client::HyperLiquidClient;

#[derive(Serialize)]
pub struct BasicUserRequest {
    #[serde(rename = "type")]
    pub request_type: String,
    pub user: String,
}

#[derive(Serialize)]
pub struct BasicRequest {
    #[serde(rename = "type")]
    pub request_type: String,
}

impl HyperLiquidClient {
    /// Internal helper method to execute requests with retry logic
    async fn execute_with_retry<F, Fut, T>(&self, operation: F) -> anyhow::Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<reqwest::Response, reqwest::Error>>,
        T: for<'de> Deserialize<'de>,
    {
        let mut last_error = None;
        
        for attempt in 0..=self.config.max_retries {
            match operation().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let result: T = response.json().await
                            .map_err(|e| anyhow::anyhow!("Failed to parse response: {}", e))?;
                        return Ok(result);
                    } else if response.status() == 429 {
                        // Rate limit - wait and retry
                        if attempt < self.config.max_retries {
                            let wait_time = Duration::from_millis(1000 * (2_u64.pow(attempt)));
                            tokio::time::sleep(wait_time).await;
                            continue;
                        } else {
                            return Err(anyhow::anyhow!("Rate limit exceeded after {} retries", self.config.max_retries));
                        }
                    } else {
                        let status = response.status();
                        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                        return Err(anyhow::anyhow!("API error: {} - {}", status, error_text));
                    }
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.config.max_retries {
                        let wait_time = Duration::from_millis(500 * (2_u64.pow(attempt)));
                        tokio::time::sleep(wait_time).await;
                        continue;
                    }
                }
            }
        }
        
        Err(anyhow::anyhow!("Request failed after {} retries: {}", 
            self.config.max_retries, 
            last_error.map(|e| e.to_string()).unwrap_or_else(|| "Unknown error".to_string())))
    }
    /// Generic method for making POST requests to the /info endpoint with user parameter
    pub async fn make_user_request<T>(&self, request_type: &str, user: &str) -> anyhow::Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/info", self.base_url);
        
        let request_body = BasicUserRequest {
            request_type: request_type.to_string(),
            user: user.to_string(),
        };

        self.execute_with_retry(|| {
            self.client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(&request_body)
                .send()
        }).await
    }
    
    /// Generic method for making POST requests to the /info endpoint without user parameter
    pub async fn make_basic_request<T>(&self, request_type: &str) -> anyhow::Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/info", self.base_url);
        
        let request_body = BasicRequest {
            request_type: request_type.to_string(),
        };

        self.execute_with_retry(|| {
            self.client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(&request_body)
                .send()
        }).await
    }
    
    /// Generic method for making custom POST requests with custom body
    pub async fn make_custom_request<B, T>(&self, body: &B) -> anyhow::Result<T>
    where
        B: Serialize,
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/info", self.base_url);

        self.execute_with_retry(|| {
            self.client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(body)
                .send()
        }).await
    }
}