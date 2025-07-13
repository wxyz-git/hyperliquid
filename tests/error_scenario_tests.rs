use hyperliquid::client::HyperLiquidClient;
use hyperliquid::config::HyperLiquidConfig;
use serde_json::json;
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[cfg(test)]
mod error_scenario_tests {
    use super::*;

    #[tokio::test]
    async fn test_input_validation_errors() {
        let client = HyperLiquidClient::new();
        
        // Test invalid Ethereum addresses
        let invalid_addresses = vec![
            "", // Empty
            "0x123", // Too short
            "1234567890123456789012345678901234567890", // No 0x prefix
            "0x123456789012345678901234567890123456789G", // Invalid hex
            "0X1234567890123456789012345678901234567890", // Wrong case prefix
        ];
        
        for invalid_addr in invalid_addresses {
            let result = client.get_portfolio(invalid_addr).await;
            assert!(result.is_err(), "Should reject invalid address: {}", invalid_addr);
            
            let error = result.unwrap_err();
            assert!(error.to_string().contains("Invalid"), "Error should mention invalid input");
        }
        
        // Test invalid coin symbols
        let invalid_coins = vec![
            "", // Empty
            "VERYLONGSYMBOLNAME", // Too long
            "BTC@USD", // Invalid character
            "BTC USD", // Space
        ];
        
        for invalid_coin in invalid_coins {
            let result = client.get_l2_book(invalid_coin).await;
            assert!(result.is_err(), "Should reject invalid coin: {}", invalid_coin);
        }
        
        // Test invalid time ranges
        let result = client.get_user_fills_by_time(
            "0x1234567890123456789012345678901234567890", 
            2000, // start_time
            Some(1000), // end_time (before start_time)
            false
        ).await;
        assert!(result.is_err(), "Should reject invalid time range");
    }

    #[tokio::test]
    async fn test_network_connection_errors() {
        // Test with completely invalid URL
        let config = HyperLiquidConfig::new()
            .with_base_url("https://this-domain-does-not-exist-12345.com")
            .with_max_retries(1)
            .with_timeout(Duration::from_secs(5));
        
        let client = HyperLiquidClient::with_config(config);
        
        let result = client.get_all_mids().await;
        assert!(result.is_err(), "Should fail with connection error");
        
        let error_message = result.unwrap_err().to_string();
        assert!(
            error_message.contains("retries") || 
            error_message.contains("connection") ||
            error_message.contains("network"),
            "Error should indicate network/connection issue"
        );
    }

    #[tokio::test]
    async fn test_malformed_json_response() {
        let mock_server = MockServer::start().await;
        
        // Return invalid JSON
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("{ invalid json content }")
                    .insert_header("content-type", "application/json")
            )
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(1);
        
        let client = HyperLiquidClient::with_config(config);
        
        let result = client.get_all_mids().await;
        assert!(result.is_err(), "Should fail with JSON parsing error");
        
        let error_message = result.unwrap_err().to_string();
        assert!(
            error_message.contains("json") || 
            error_message.contains("parse") ||
            error_message.contains("deserialize"),
            "Error should indicate JSON parsing issue"
        );
    }

    #[tokio::test]
    async fn test_unexpected_response_structure() {
        let mock_server = MockServer::start().await;
        
        // Return valid JSON but wrong structure
        let wrong_response = json!({
            "unexpected_field": "value",
            "missing_required_fields": true
        });
        
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&wrong_response))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(1);
        
        let client = HyperLiquidClient::with_config(config);
        
        let result = client.get_all_mids().await;
        assert!(result.is_err(), "Should fail with deserialization error");
    }

    #[tokio::test]
    async fn test_various_http_error_codes() {
        let error_codes_and_messages = vec![
            (400, "Bad Request"),
            (401, "Unauthorized"), 
            (403, "Forbidden"),
            (404, "Not Found"),
            (422, "Unprocessable Entity"),
            (500, "Internal Server Error"),
            (502, "Bad Gateway"),
            (503, "Service Unavailable"),
        ];
        
        for (status_code, expected_context) in error_codes_and_messages {
            let mock_server = MockServer::start().await;
            
            Mock::given(method("POST"))
                .and(path("/info"))
                .respond_with(
                    ResponseTemplate::new(status_code)
                        .set_body_string(format!("API Error: {}", expected_context))
                )
                .mount(&mock_server)
                .await;
            
            let config = HyperLiquidConfig::new()
                .with_base_url(&mock_server.uri())
                .with_max_retries(1)
                .with_timeout(Duration::from_secs(5));
            
            let client = HyperLiquidClient::with_config(config);
            
            let result = client.get_all_mids().await;
            assert!(result.is_err(), "Status {} should result in error", status_code);
            
            // Verify error contains meaningful information
            let error_message = result.unwrap_err().to_string();
            assert!(
                !error_message.is_empty(),
                "Error message should not be empty for status {}", 
                status_code
            );
        }
    }

    #[tokio::test]
    async fn test_timeout_scenarios() {
        let mock_server = MockServer::start().await;
        
        // Simulate very slow response
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_delay(Duration::from_secs(30)) // Much longer than timeout
                    .set_body_json(&json!({"test": "data"}))
            )
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(1)
            .with_timeout(Duration::from_secs(2)); // Short timeout
        
        let client = HyperLiquidClient::with_config(config);
        
        let start = std::time::Instant::now();
        let result = client.get_all_mids().await;
        let elapsed = start.elapsed();
        
        assert!(result.is_err(), "Should timeout");
        assert!(elapsed < Duration::from_secs(10), "Should not wait for full response delay");
        
        let error_message = result.unwrap_err().to_string();
        assert!(
            error_message.contains("timeout") || 
            error_message.contains("retries"),
            "Error should indicate timeout"
        );
    }

    #[tokio::test]
    async fn test_empty_response_body() {
        let mock_server = MockServer::start().await;
        
        // Return 200 but empty body
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("")
                    .insert_header("content-type", "application/json")
            )
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri());
        
        let client = HyperLiquidClient::with_config(config);
        
        let result = client.get_all_mids().await;
        assert!(result.is_err(), "Should fail with empty response body");
    }

    #[tokio::test]
    async fn test_concurrent_error_handling() {
        let mock_server = MockServer::start().await;
        
        // Simple test: all concurrent requests should eventually succeed
        // Some will succeed immediately, others will retry once and then succeed
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(500)) // Server error - will be retried
            .up_to_n_times(3) // First 3 calls fail
            .mount(&mock_server)
            .await;
        
        // All subsequent requests succeed (including retries)
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&json!({"test": "success"})))
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_max_retries(2) // Allow retries
            .with_timeout(Duration::from_secs(10));
        
        let client = HyperLiquidClient::with_config(config);
        
        // Make 3 concurrent requests
        let mut handles = vec![];
        for _i in 0..3 {
            let client_clone = client.clone();
            let handle = tokio::spawn(async move {
                client_clone.get_all_mids().await
            });
            handles.push(handle);
        }
        
        let results: Vec<_> = future::join_all(handles).await;
        
        let errors = results.iter().filter(|r: &&Result<_, _>| r.as_ref().unwrap().is_err()).count();
        let successes = results.iter().filter(|r: &&Result<_, _>| r.as_ref().unwrap().is_ok()).count();
        
        // With enough retries, all requests should eventually succeed
        // This tests that concurrent requests with retries work correctly
        assert_eq!(errors, 0, "All requests should succeed after retries");
        assert_eq!(successes, 3, "All 3 concurrent requests should succeed");
    }

    #[tokio::test]
    async fn test_error_message_quality() {
        let client = HyperLiquidClient::new();
        
        // Test that error messages are descriptive and actionable
        let invalid_addr = "invalid_address";
        let result = client.get_portfolio(invalid_addr).await;
        
        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        
        // Error message should be descriptive
        assert!(error_message.len() > 20, "Error message should be descriptive");
        assert!(error_message.contains("address"), "Error should mention address");
        assert!(error_message.contains("Invalid"), "Error should indicate what's invalid");
        
        // Should not contain internal implementation details
        assert!(!error_message.contains("unwrap"), "Error should not expose internal details");
        assert!(!error_message.contains("panic"), "Error should not expose internal details");
    }

    #[tokio::test]
    async fn test_connection_pool_exhaustion() {
        // This test verifies graceful handling when connection pool is exhausted
        let mock_server = MockServer::start().await;
        
        Mock::given(method("POST"))
            .and(path("/info"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_delay(Duration::from_secs(1)) // Slow responses to hold connections
                    .set_body_json(&json!({"test": "data"}))
            )
            .mount(&mock_server)
            .await;
        
        let config = HyperLiquidConfig::new()
            .with_base_url(&mock_server.uri())
            .with_timeout(Duration::from_secs(5));
        
        let client = HyperLiquidClient::with_config(config);
        
        // Create many concurrent requests to potentially exhaust connection pool
        let mut handles = vec![];
        for _ in 0..50 { // More than typical connection pool size
            let client_clone = client.clone();
            let handle = tokio::spawn(async move {
                client_clone.get_all_mids().await
            });
            handles.push(handle);
        }
        
        let results: Vec<_> = future::join_all(handles).await;
        
        // Most requests should succeed (connection pooling should handle this gracefully)
        let successful_requests = results.iter()
            .filter(|r: &&Result<_, _>| r.as_ref().unwrap().is_ok())
            .count();
        
        assert!(successful_requests > 40, "Connection pooling should handle concurrent requests");
    }
}

use futures::future;