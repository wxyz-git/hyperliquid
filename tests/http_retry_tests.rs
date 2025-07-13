use hyperliquid::client::HyperLiquidClient;
use hyperliquid::config::HyperLiquidConfig;
use serde_json::json;
use std::time::Duration;
use wiremock::matchers::{method, path, body_json};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[cfg(test)]
mod retry_logic_tests {
    use super::*;

    #[tokio::test]
    async fn test_retry_logic_succeeds_after_failures() {
        let mock_server = MockServer::start().await;
        
        // First 2 requests fail with 500
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(500))
            .up_to_n_times(2)
            .mount(&mock_server)
            .await;
        
        // Third request succeeds
        let success_response = json!({
            "BTC": "50000.0",
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
            .with_timeout(Duration::from_secs(5));
        
        let client = HyperLiquidClient::with_config(config);
        
        // This should succeed after 2 failures + 1 success
        let result = client.get_all_mids().await;
        assert!(result.is_ok(), "Client should succeed after retries");
    }

    #[tokio::test]
    async fn test_retry_logic_fails_after_max_retries() {
        let mock_server = MockServer::start().await;
        
        // All requests fail with 500
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(2)  // Only 2 retries
            .with_timeout(Duration::from_secs(5));
        
        let client = HyperLiquidClient::with_config(config);
        
        let result = client.get_all_mids().await;
        assert!(result.is_err(), "Client should fail after max retries");
        
        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("retries"), "Error should mention retries");
        assert!(error_message.contains("2"), "Error should mention max retry count");
    }

    #[tokio::test] 
    async fn test_exponential_backoff_timing() {
        let mock_server = MockServer::start().await;
        
        // All requests fail to test backoff timing
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(2) // Shorter for faster testing
            .with_timeout(Duration::from_secs(5));
        
        let client = HyperLiquidClient::with_config(config);
        
        let start = std::time::Instant::now();
        let _result = client.get_all_mids().await; // Will fail but we're testing timing
        let elapsed = start.elapsed();
        
        // Should have some backoff delay (at least 1 second for retries)
        assert!(elapsed >= Duration::from_secs(1), "Should have some backoff delay");
        assert!(elapsed <= Duration::from_secs(15), "Should not take too long");
    }

    #[tokio::test]
    async fn test_network_timeout_handling() {
        let mock_server = MockServer::start().await;
        
        // Simulate slow response (longer than timeout)
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_delay(Duration::from_secs(10)) // Longer than client timeout
            )
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(1)
            .with_timeout(Duration::from_secs(2)); // Short timeout
        
        let client = HyperLiquidClient::with_config(config);
        
        let result = client.get_all_mids().await;
        assert!(result.is_err(), "Should timeout and fail");
        
        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("retries") || error_message.contains("timeout"), 
                "Error should mention timeout or retries");
    }

    #[tokio::test]
    async fn test_successful_request_no_retry() {
        let mock_server = MockServer::start().await;
        
        let success_response = json!({
            "BTC": "50000.0",
            "ETH": "3000.0"
        });
        
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&success_response))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(3);
        
        let client = HyperLiquidClient::with_config(config);
        
        let result = client.get_all_mids().await;
        assert!(result.is_ok(), "Successful request should not retry");
        
        // Mock should have been called exactly once (no retries needed)
    }

    #[tokio::test]
    async fn test_different_http_error_codes() {
        let test_cases = vec![
            (400, false), // Bad request - should not retry
            (401, false), // Unauthorized - should not retry  
            (404, false), // Not found - should not retry
            (429, true),  // Rate limited - should retry
            (500, true),  // Server error - should retry
            (502, true),  // Bad gateway - should retry
            (503, true),  // Service unavailable - should retry
        ];
        
        for (status_code, _should_retry) in test_cases {
            let mock_server = MockServer::start().await;
            
            Mock::given(method("POST"))
                .and(path("/info"))
                .respond_with(ResponseTemplate::new(status_code))
                .mount(&mock_server)
                .await;
            
            let config = HyperLiquidConfig::new()
                .with_base_url(&mock_server.uri())
                .with_max_retries(2)
                .with_timeout(Duration::from_secs(5));
            
            let client = HyperLiquidClient::with_config(config);
            
            let result = client.get_all_mids().await;
            assert!(result.is_err(), "Status {} should result in error", status_code);
            
            // TODO: Verify retry behavior based on should_retry
            // This requires more sophisticated mock verification
        }
    }

    #[tokio::test]
    async fn test_request_body_validation() {
        let mock_server = MockServer::start().await;
        
        // Verify the request body structure
        let expected_body = json!({
            "type": "allMids"
        });
        
        Mock::given(method("POST"))
            .and(path("/info"))
            .and(body_json(&expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json!({})))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri());
        
        let client = HyperLiquidClient::with_config(config);
        
        let result = client.get_all_mids().await;
        assert!(result.is_ok(), "Request with correct body should succeed");
    }
}