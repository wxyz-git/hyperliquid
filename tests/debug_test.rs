use hyperliquid::client::HyperLiquidClient;
use hyperliquid::config::HyperLiquidConfig;
use serde_json::json;
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[cfg(test)]
mod debug_tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_success() {
        let mock_server = MockServer::start().await;
        
        // Simple success response
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json!({"BTC": "50000"})))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(1)
            .with_timeout(Duration::from_secs(5));
        
        let client = HyperLiquidClient::with_config(config);
        
        let result = client.get_all_mids().await;
        match &result {
            Ok(_) => println!("✅ Success!"),
            Err(e) => println!("❌ Error: {}", e),
        }
        
        assert!(result.is_ok(), "Simple request should succeed");
    }

    #[tokio::test]
    async fn test_single_retry_scenario() {
        let mock_server = MockServer::start().await;
        
        // First request fails
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(500))
            .up_to_n_times(1)
            .mount(&mock_server)
            .await;
        
        // Subsequent requests succeed
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json!({"BTC": "50000"})))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(2)
            .with_timeout(Duration::from_secs(10));
        
        let client = HyperLiquidClient::with_config(config);
        
        println!("Starting retry test...");
        let result = client.get_all_mids().await;
        
        match &result {
            Ok(_) => println!("✅ Retry succeeded!"),
            Err(e) => println!("❌ Retry failed: {}", e),
        }
        
        // This should succeed after retry
        assert!(result.is_ok(), "Should succeed after one retry");
    }
    
    #[tokio::test]
    async fn test_debug_concurrent_simple() {
        let mock_server = MockServer::start().await;
        
        // All requests succeed
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json!({"BTC": "50000"})))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(1)
            .with_timeout(Duration::from_secs(5));
        
        let client = HyperLiquidClient::with_config(config);
        
        // Make 3 simple concurrent requests
        let mut handles = vec![];
        for i in 0..3 {
            let client_clone = client.clone();
            let handle = tokio::spawn(async move {
                println!("Request {} starting", i);
                let result = client_clone.get_all_mids().await;
                println!("Request {} result: {:?}", i, result.is_ok());
                result
            });
            handles.push(handle);
        }
        
        let results: Vec<_> = futures::future::join_all(handles).await;
        
        let successes = results.iter().filter(|r| r.as_ref().unwrap().is_ok()).count();
        println!("Successes: {}/{}", successes, results.len());
        
        assert_eq!(successes, 3, "All simple concurrent requests should succeed");
    }
}

use futures;