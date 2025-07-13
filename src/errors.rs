use thiserror::Error;

#[derive(Error, Debug)]
pub enum HyperLiquidError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("API error: {status} - {message}")]
    ApiError { status: u16, message: String },
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Invalid Ethereum address: {0}")]
    InvalidAddress(String),
    
    #[error("Invalid coin symbol: {0}")]
    InvalidCoin(String),
    
    #[error("Invalid time range: start_time ({start}) must be less than end_time ({end})")]
    InvalidTimeRange { start: u64, end: u64 },
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Timeout occurred")]
    Timeout,
}

pub type Result<T> = std::result::Result<T, HyperLiquidError>;

pub fn validate_ethereum_address(address: &str) -> Result<()> {
    if !address.starts_with("0x") {
        return Err(HyperLiquidError::InvalidAddress(
            format!("Invalid Ethereum address '{}': must start with '0x'", address)
        ));
    }
    
    if address.len() != 42 {
        return Err(HyperLiquidError::InvalidAddress(
            format!("Invalid Ethereum address '{}': must be 42 characters long, got {} characters", 
                   address, address.len())
        ));
    }
    
    if !address[2..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(HyperLiquidError::InvalidAddress(
            format!("Invalid Ethereum address '{}': contains non-hexadecimal characters", address)
        ));
    }
    
    Ok(())
}

pub fn validate_coin_symbol(coin: &str) -> Result<()> {
    if coin.is_empty() {
        return Err(HyperLiquidError::InvalidCoin(
            "Coin symbol cannot be empty".to_string()
        ));
    }
    
    if coin.len() > 10 {
        return Err(HyperLiquidError::InvalidCoin(
            format!("Invalid coin symbol '{}': too long (max 10 characters, got {})", coin, coin.len())
        ));
    }
    
    if !coin.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(HyperLiquidError::InvalidCoin(
            format!("Invalid coin symbol '{}': contains invalid characters (only alphanumeric, '-', and '_' allowed)", coin)
        ));
    }
    
    Ok(())
}

pub fn validate_time_range(start_time: u64, end_time: u64) -> Result<()> {
    if start_time >= end_time {
        return Err(HyperLiquidError::InvalidTimeRange {
            start: start_time,
            end: end_time,
        });
    }
    
    Ok(())
}

pub fn validate_interval(interval: &str) -> Result<()> {
    let valid_intervals = ["1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "1w", "1M"];
    
    if !valid_intervals.contains(&interval) {
        return Err(HyperLiquidError::InvalidInput(
            format!("Invalid interval '{}'. Valid intervals are: {}", interval, valid_intervals.join(", "))
        ));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethereum_address_validation() {
        // Valid cases
        assert!(validate_ethereum_address("0x1234567890123456789012345678901234567890").is_ok());
        assert!(validate_ethereum_address("0xabcdefABCDEF1234567890123456789012345678").is_ok());
        assert!(validate_ethereum_address("0x0000000000000000000000000000000000000000").is_ok());

        // Invalid cases
        assert!(validate_ethereum_address("").is_err());
        assert!(validate_ethereum_address("0x123").is_err());
        assert!(validate_ethereum_address("1234567890123456789012345678901234567890").is_err());
        assert!(validate_ethereum_address("0x123456789012345678901234567890123456789G").is_err());
        assert!(validate_ethereum_address("0X1234567890123456789012345678901234567890").is_err());
    }

    #[test]
    fn test_coin_symbol_validation() {
        // Valid cases
        assert!(validate_coin_symbol("BTC").is_ok());
        assert!(validate_coin_symbol("ETH").is_ok());
        assert!(validate_coin_symbol("BTC-USD").is_ok());
        assert!(validate_coin_symbol("ETH_USDC").is_ok());
        assert!(validate_coin_symbol("A").is_ok());

        // Invalid cases
        assert!(validate_coin_symbol("").is_err());
        assert!(validate_coin_symbol("VERYLONGSYMBOL").is_err());
        assert!(validate_coin_symbol("BTC@USD").is_err());
        assert!(validate_coin_symbol("BTC USD").is_err());
        assert!(validate_coin_symbol("BTC/USD").is_err());
    }

    #[test]
    fn test_interval_validation() {
        // Valid intervals
        let valid_intervals = ["1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "1w", "1M"];
        for interval in valid_intervals.iter() {
            assert!(validate_interval(interval).is_ok(), "Failed for interval: {}", interval);
        }

        // Invalid intervals
        assert!(validate_interval("").is_err());
        assert!(validate_interval("2m").is_err());
        assert!(validate_interval("1x").is_err());
        assert!(validate_interval("1 m").is_err());
        assert!(validate_interval("60m").is_err());
    }

    #[test]
    fn test_time_range_validation() {
        // Valid ranges
        assert!(validate_time_range(100, 200).is_ok());
        assert!(validate_time_range(0, 1).is_ok());
        assert!(validate_time_range(1609459200000, 1609545600000).is_ok()); // Real timestamps

        // Invalid ranges
        assert!(validate_time_range(200, 100).is_err());
        assert!(validate_time_range(100, 100).is_err());
        assert!(validate_time_range(u64::MAX, 0).is_err());
    }

    #[test]
    fn test_error_display() {
        let err = HyperLiquidError::InvalidAddress("test".to_string());
        assert!(err.to_string().contains("Invalid Ethereum address"));

        let err = HyperLiquidError::InvalidTimeRange { start: 100, end: 50 };
        assert!(err.to_string().contains("Invalid time range"));

        let err = HyperLiquidError::RateLimitExceeded;
        assert_eq!(err.to_string(), "Rate limit exceeded");
    }
}