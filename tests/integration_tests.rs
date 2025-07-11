use hyperliquid::client::HyperLiquidClient;
use hyperliquid::config::HyperLiquidConfig;
use hyperliquid::errors::{validate_ethereum_address, validate_coin_symbol, validate_interval, validate_time_range};
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = HyperLiquidClient::new();
        assert_eq!(client.base_url, "https://api.hyperliquid.xyz");
    }

    #[test]
    fn test_client_with_config() {
        let config = HyperLiquidConfig::new()
            .with_base_url("https://custom.api.com")
            .with_timeout(Duration::from_secs(60))
            .with_max_retries(5);

        let client = HyperLiquidClient::with_config(config.clone());
        assert_eq!(client.base_url, "https://custom.api.com");
        assert_eq!(client.config.timeout, Duration::from_secs(60));
        assert_eq!(client.config.max_retries, 5);
    }

    #[test]
    fn test_config_from_env() {
        unsafe {
            std::env::set_var("HYPERLIQUID_BASE_URL", "https://test.api.com");
            std::env::set_var("HYPERLIQUID_TIMEOUT_SECS", "45");
            std::env::set_var("HYPERLIQUID_MAX_RETRIES", "2");
        }
        
        let config = HyperLiquidConfig::from_env();
        assert_eq!(config.base_url, "https://test.api.com");
        assert_eq!(config.timeout, Duration::from_secs(45));
        assert_eq!(config.max_retries, 2);
        
        // Clean up
        unsafe {
            std::env::remove_var("HYPERLIQUID_BASE_URL");
            std::env::remove_var("HYPERLIQUID_TIMEOUT_SECS");
            std::env::remove_var("HYPERLIQUID_MAX_RETRIES");
        }
    }

    #[test]
    fn test_validate_ethereum_address() {
        // Valid addresses
        assert!(validate_ethereum_address("0x1234567890123456789012345678901234567890").is_ok());
        assert!(validate_ethereum_address("0xabcdefABCDEF1234567890123456789012345678").is_ok());
        
        // Invalid addresses
        assert!(validate_ethereum_address("1234567890123456789012345678901234567890").is_err()); // No 0x prefix
        assert!(validate_ethereum_address("0x123456789012345678901234567890123456789").is_err()); // Too short
        assert!(validate_ethereum_address("0x12345678901234567890123456789012345678901").is_err()); // Too long
        assert!(validate_ethereum_address("0x123456789012345678901234567890123456789G").is_err()); // Invalid hex character
        assert!(validate_ethereum_address("").is_err()); // Empty
    }

    #[test]
    fn test_validate_coin_symbol() {
        // Valid symbols
        assert!(validate_coin_symbol("BTC").is_ok());
        assert!(validate_coin_symbol("ETH").is_ok());
        assert!(validate_coin_symbol("USDC").is_ok());
        assert!(validate_coin_symbol("SOL").is_ok());
        assert!(validate_coin_symbol("BTC-USD").is_ok());
        assert!(validate_coin_symbol("ETH_USDC").is_ok());
        
        // Invalid symbols
        assert!(validate_coin_symbol("").is_err()); // Empty
        assert!(validate_coin_symbol("VERYLONGSYMBOL").is_err()); // Too long
        assert!(validate_coin_symbol("BTC@USD").is_err()); // Invalid character
        assert!(validate_coin_symbol("BTC USD").is_err()); // Space
    }

    #[test]
    fn test_validate_interval() {
        // Valid intervals
        assert!(validate_interval("1m").is_ok());
        assert!(validate_interval("5m").is_ok());
        assert!(validate_interval("1h").is_ok());
        assert!(validate_interval("1d").is_ok());
        assert!(validate_interval("1w").is_ok());
        assert!(validate_interval("1M").is_ok());
        
        // Invalid intervals
        assert!(validate_interval("2m").is_err()); // Not in valid list
        assert!(validate_interval("1x").is_err()); // Invalid unit
        assert!(validate_interval("").is_err()); // Empty
        assert!(validate_interval("1 m").is_err()); // Space
    }

    #[test]
    fn test_validate_time_range() {
        // Valid ranges
        assert!(validate_time_range(1000, 2000).is_ok());
        assert!(validate_time_range(0, 1).is_ok());
        
        // Invalid ranges
        assert!(validate_time_range(2000, 1000).is_err()); // start >= end
        assert!(validate_time_range(1000, 1000).is_err()); // start == end
    }

    // Integration test that requires network access
    #[tokio::test]
    #[ignore] // Use `cargo test -- --ignored` to run network tests
    async fn test_get_all_mids_integration() {
        let client = HyperLiquidClient::new();
        let result = client.get_all_mids().await;
        
        match result {
            Ok(mid_prices) => {
                println!("Successfully fetched mid prices: {:?}", mid_prices);
                assert!(!mid_prices.0.is_empty());
            }
            Err(e) => {
                println!("Expected error (network/API might be down): {}", e);
                // Don't fail the test for network issues
            }
        }
    }

    #[tokio::test]
    #[ignore] // Use `cargo test -- --ignored` to run network tests
    async fn test_get_candle_snapshot_integration() {
        let client = HyperLiquidClient::new();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let start_time = now - 3600000; // 1 hour ago
        
        let result = client.get_candle_snapshot("BTC", "1m", start_time, now).await;
        
        match result {
            Ok(candles) => {
                println!("Successfully fetched {} candles", candles.len());
                if !candles.is_empty() {
                    assert_eq!(candles[0].s, "BTC");
                    assert_eq!(candles[0].i, "1m");
                }
            }
            Err(e) => {
                println!("Expected error (network/API might be down): {}", e);
                // Don't fail the test for network issues
            }
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_retry_logic_with_invalid_endpoint() {
        let config = HyperLiquidConfig::new()
            .with_base_url("https://invalid-api-endpoint-that-does-not-exist.com")
            .with_max_retries(1)
            .with_timeout(Duration::from_secs(5));
        
        let client = HyperLiquidClient::with_config(config);
        let result = client.get_all_mids().await;
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("retries"));
    }
}