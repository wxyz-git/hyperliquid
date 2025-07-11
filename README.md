# HyperLiquid Rust API Client

A robust, production-ready Rust client library for the HyperLiquid API with comprehensive error handling, input validation, retry logic, and extensive testing.

## Features

- ✅ **Complete API Coverage**: All HyperLiquid API endpoints
- ✅ **Type Safety**: Fully typed requests and responses with serde
- ✅ **Input Validation**: Validates Ethereum addresses, coin symbols, intervals, and time ranges
- ✅ **Retry Logic**: Automatic retries with exponential backoff for failed requests
- ✅ **Rate Limiting**: Built-in handling for API rate limits (HTTP 429)
- ✅ **Connection Pooling**: Efficient HTTP connection reuse and keep-alive
- ✅ **Configuration**: Environment-based configuration and builder pattern
- ✅ **Error Handling**: Comprehensive error types with context
- ✅ **Testing**: Extensive unit and integration tests
- ✅ **Documentation**: Complete API documentation with examples

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
hyperliquid = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
```

### Basic Usage

```rust
use hyperliquid::client::HyperLiquidClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = HyperLiquidClient::new();
    
    // Get all mid prices
    let mid_prices = client.get_all_mids().await?;
    println!("Mid prices: {:?}", mid_prices);
    
    // Get user portfolio (replace with actual address)
    let user_address = "0x1234567890123456789012345678901234567890";
    let portfolio = client.get_portfolio(user_address).await?;
    println!("Portfolio: {:?}", portfolio);
    
    Ok(())
}
```

### Configuration

#### Using Builder Pattern

```rust
use hyperliquid::{client::HyperLiquidClient, config::HyperLiquidConfig};
use std::time::Duration;

let config = HyperLiquidConfig::new()
    .with_base_url("https://api.hyperliquid.xyz")
    .with_timeout(Duration::from_secs(30))
    .with_max_retries(3)
    .with_compression(true);

let client = HyperLiquidClient::with_config(config);
```

#### Using Environment Variables

```bash
export HYPERLIQUID_BASE_URL="https://api.hyperliquid.xyz"
export HYPERLIQUID_TIMEOUT_SECS="30"
export HYPERLIQUID_MAX_RETRIES="3"
export HYPERLIQUID_ENABLE_COMPRESSION="true"
```

```rust
let client = HyperLiquidClient::from_env();
```

## API Examples

### Market Data

```rust
// Get L2 order book
let l2_book = client.get_l2_book("BTC").await?;

// Get candle data
let now = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)?
    .as_millis() as u64;
let start_time = now - 3600000; // 1 hour ago

let candles = client.get_candle_snapshot("BTC", "1m", start_time, now).await?;
```

### User Data

```rust
let user_address = "0x1234567890123456789012345678901234567890";

// Get open orders
let orders = client.get_open_orders(user_address).await?;

// Get user fills
let fills = client.get_user_fills(user_address).await?;

// Get user rate limit
let rate_limit = client.get_user_rate_limit(user_address).await?;
```

### Perpetuals

```rust
// Get available perpetual DEXs
let perp_dexs = client.get_perp_dexs().await?;

// Get max builder fee
let max_fee = client.get_max_builder_fee(user_address, "builder_name").await?;
```

## Error Handling

The library provides comprehensive error handling with custom error types:

```rust
use hyperliquid::errors::HyperLiquidError;

match client.get_portfolio("invalid_address").await {
    Ok(portfolio) => println!("Portfolio: {:?}", portfolio),
    Err(e) => match e.downcast_ref::<HyperLiquidError>() {
        Some(HyperLiquidError::InvalidAddress(addr)) => {
            println!("Invalid address: {}", addr);
        }
        Some(HyperLiquidError::RateLimitExceeded) => {
            println!("Rate limited - try again later");
        }
        _ => println!("Other error: {}", e),
    }
}
```

## Input Validation

All inputs are validated before making API requests:

```rust
// These will return validation errors
client.get_portfolio("invalid_address").await?; // Invalid Ethereum address
client.get_candle_snapshot("", "1m", 0, 1).await?; // Empty coin symbol
client.get_candle_snapshot("BTC", "2m", 0, 1).await?; // Invalid interval
client.get_candle_snapshot("BTC", "1m", 100, 50).await?; // Invalid time range
```

## Testing

Run the test suite:

```bash
# Unit tests
cargo test --lib

# Integration tests (requires network)
cargo test --test integration_tests

# All tests including network tests
cargo test -- --ignored
```

## Configuration Options

| Option | Environment Variable | Default | Description |
|--------|---------------------|---------|-------------|
| `base_url` | `HYPERLIQUID_BASE_URL` | `https://api.hyperliquid.xyz` | API base URL |
| `timeout` | `HYPERLIQUID_TIMEOUT_SECS` | `30` | Request timeout in seconds |
| `max_retries` | `HYPERLIQUID_MAX_RETRIES` | `3` | Maximum retry attempts |
| `enable_compression` | `HYPERLIQUID_ENABLE_COMPRESSION` | `true` | Enable gzip compression |
| `user_agent` | `HYPERLIQUID_USER_AGENT` | `hyperliquid-rust-client/0.1.0` | HTTP User-Agent |

## Performance Features

- **Connection Pooling**: Reuses HTTP connections for better performance
- **Keep-Alive**: TCP keep-alive enabled by default
- **Compression**: Gzip compression for reduced bandwidth
- **Retry Logic**: Exponential backoff for transient failures
- **Input Validation**: Client-side validation prevents unnecessary API calls

## Error Types

| Error | Description |
|-------|-------------|
| `HttpError` | Network or HTTP errors |
| `InvalidInput` | Invalid input parameters |
| `ApiError` | API returned an error status |
| `SerializationError` | JSON parsing errors |
| `InvalidAddress` | Invalid Ethereum address |
| `InvalidCoin` | Invalid coin symbol |
| `InvalidTimeRange` | Invalid time range |
| `RateLimitExceeded` | API rate limit exceeded |
| `Timeout` | Request timeout |

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for your changes
5. Ensure all tests pass (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This is an unofficial client library. Always verify your trades and use at your own risk. The authors are not responsible for any trading losses.