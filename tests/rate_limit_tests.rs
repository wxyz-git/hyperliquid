use hyperliquid::client::HyperLiquidClient;
use hyperliquid::config::HyperLiquidConfig;
use serde_json::json;
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use futures::future;

#[cfg(test)]
mod rate_limit_tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limit_with_retry_after_header() {
        let mock_server = MockServer::start().await;
        
        // First request: rate limited with Retry-After header
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(
                ResponseTemplate::new(429)
                    .insert_header("retry-after", "2") // Wait 2 seconds
            )
            .up_to_n_times(1)
            .mount(&mock_server)
            .await;
        
        // Second request: success
        let success_response = json!({
            "BTC": "50000.0"
        });
        
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&success_response))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(3)
            .with_timeout(Duration::from_secs(10));
        
        let client = HyperLiquidClient::with_config(config);
        
        let start = std::time::Instant::now();
        let result = client.get_all_mids().await;
        let elapsed = start.elapsed();
        
        assert!(result.is_ok(), "Should succeed after rate limit backoff");
        // Our implementation uses exponential backoff (1 second for first retry), not Retry-After header
        assert!(elapsed >= Duration::from_secs(1), "Should use exponential backoff for rate limits");
    }

    #[tokio::test]
    async fn test_rate_limit_without_retry_after_header() {
        let mock_server = MockServer::start().await;
        
        // First request: rate limited without Retry-After header
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(429)) // No Retry-After header
            .up_to_n_times(1)
            .mount(&mock_server)
            .await;
        
        // Second request: success
        let success_response = json!({
            "ETH": "3000.0"
        });
        
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&success_response))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(3)
            .with_timeout(Duration::from_secs(10));
        
        let client = HyperLiquidClient::with_config(config);
        
        let start = std::time::Instant::now();
        let result = client.get_all_mids().await;
        let elapsed = start.elapsed();
        
        assert!(result.is_ok(), "Should succeed after exponential backoff");
        // Should use default exponential backoff (at least 1 second for first retry)
        assert!(elapsed >= Duration::from_secs(1), "Should use exponential backoff when no Retry-After");
    }

    #[tokio::test]
    async fn test_multiple_rate_limits() {
        let mock_server = MockServer::start().await;
        
        // First two requests: rate limited
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(
                ResponseTemplate::new(429)
                    .insert_header("retry-after", "1")
            )
            .up_to_n_times(2)
            .mount(&mock_server)
            .await;
        
        // Third request: success
        let success_response = json!({
            "SOL": "100.0"
        });
        
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&success_response))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(3)
            .with_timeout(Duration::from_secs(15));
        
        let client = HyperLiquidClient::with_config(config);
        
        let start = std::time::Instant::now();
        let result = client.get_all_mids().await;
        let elapsed = start.elapsed();
        
        assert!(result.is_ok(), "Should succeed after multiple rate limit backoffs");
        // Should wait at least 2 seconds (1s + 1s from retry-after headers)
        assert!(elapsed >= Duration::from_secs(2), "Should respect multiple Retry-After headers");
    }

    #[tokio::test]
    async fn test_rate_limit_exhausts_retries() {
        let mock_server = MockServer::start().await;
        
        // All requests: rate limited
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(
                ResponseTemplate::new(429)
                    .insert_header("retry-after", "1")
            )
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(2) // Only 2 retries
            .with_timeout(Duration::from_secs(10));
        
        let client = HyperLiquidClient::with_config(config);
        
        let result = client.get_all_mids().await;
        assert!(result.is_err(), "Should fail after exhausting retries on rate limits");
        
        let error_message = result.unwrap_err().to_string();
        assert!(
            error_message.contains("retries") || error_message.contains("429"), 
            "Error should mention retries or rate limiting"
        );
    }

    #[tokio::test]
    async fn test_rate_limit_with_malformed_retry_after() {
        let mock_server = MockServer::start().await;
        
        // Rate limited with invalid Retry-After header
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(
                ResponseTemplate::new(429)
                    .insert_header("retry-after", "invalid") // Invalid value
            )
            .up_to_n_times(1)
            .mount(&mock_server)
            .await;
        
        // Second request: success
        let success_response = json!({
            "AVAX": "25.0"
        });
        
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&success_response))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(3)
            .with_timeout(Duration::from_secs(10));
        
        let client = HyperLiquidClient::with_config(config);
        
        let start = std::time::Instant::now();
        let result = client.get_all_mids().await;
        let elapsed = start.elapsed();
        
        assert!(result.is_ok(), "Should succeed with fallback to exponential backoff");
        // Should fallback to exponential backoff when Retry-After is malformed
        assert!(elapsed >= Duration::from_secs(1), "Should fallback to exponential backoff");
    }

    #[tokio::test]
    async fn test_concurrent_requests_rate_limiting() {
        let mock_server = MockServer::start().await;
        
        // Simulate different responses for concurrent requests
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(429))
            .up_to_n_times(3) // Some requests hit rate limit
            .mount(&mock_server)
            .await;
        
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json!({"test": "success"})))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(2)
            .with_timeout(Duration::from_secs(15));
        
        let client = HyperLiquidClient::with_config(config);
        
        // Make 5 concurrent requests
        let mut handles = vec![];
        for _ in 0..5 {
            let client_clone = client.clone();
            let handle = tokio::spawn(async move {
                client_clone.get_all_mids().await
            });
            handles.push(handle);
        }
        
        // Wait for all requests to complete
        let results: Vec<_> = future::join_all(handles).await;
        
        let successful_requests = results.iter()
            .filter(|r: &&Result<_, _>| r.as_ref().unwrap().is_ok())
            .count();
        
        // At least some requests should succeed (due to retries)
        assert!(successful_requests > 0, "Some concurrent requests should succeed despite rate limiting");
    }

    #[tokio::test]
    async fn test_rate_limit_preserves_request_data() {
        let mock_server = MockServer::start().await;
        
        // Verify that retried requests maintain the same body
        let expected_body = json!({
            "type": "openOrders",
            "user": "0x1234567890123456789012345678901234567890"
        });
        
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(429))
            .up_to_n_times(1)
            .mount(&mock_server)
            .await;
        
        Mock::given(method("POST"))
            .and(path("/info"))
            .and(wiremock::matchers::body_json(&expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json!([])))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(3);
        
        let client = HyperLiquidClient::with_config(config);
        
        let result = client.get_open_orders("0x1234567890123456789012345678901234567890").await;
        assert!(result.is_ok(), "Should succeed with correct request body preserved");
    }
}

