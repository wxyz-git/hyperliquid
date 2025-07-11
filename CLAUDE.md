# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Development Commands

### Building and Testing
- `cargo build` - Build the project
- `cargo test` - Run unit tests only
- `cargo test --test integration_tests` - Run integration tests (requires network)
- `cargo test -- --ignored` - Run all tests including network tests
- `cargo check` - Fast syntax and type checking
- `cargo clippy` - Run the linter
- `cargo fmt` - Format code

### Testing Strategy
- Unit tests are in individual modules (e.g., `tests` modules in source files)
- Integration tests are in `tests/integration_tests.rs`
- Network-dependent tests are marked with `#[ignore]` - use `cargo test -- --ignored` to run them
- Test addresses use valid Ethereum format but are examples (not real trading addresses)

## Project Architecture

### Core Components

**Client Architecture (`src/client.rs`)**
- `HyperLiquidClient` - Main API client with connection pooling and retry logic
- Thread-safe design using `Arc<HyperLiquidConfig>`
- Built on reqwest with automatic retries, rate limiting, and connection reuse

**Configuration (`src/config.rs`)**
- `HyperLiquidConfig` - Builder pattern configuration
- Environment variable support (prefix: `HYPERLIQUID_`)
- Default values optimized for production use

**Error Handling (`src/errors.rs`)**
- `HyperLiquidError` enum with specific error types
- Input validation functions for addresses, coin symbols, intervals, and time ranges
- Comprehensive validation prevents invalid API calls

**Common Utilities (`src/common.rs`)**
- Generic request methods: `make_user_request`, `make_basic_request`, `make_custom_request`
- Automatic retry logic with exponential backoff
- Rate limit handling (HTTP 429) with backoff

### API Modules (`src/info/`)
Each module in `src/info/` corresponds to a specific HyperLiquid API endpoint:
- Market data: `all_mids`, `candle_snapshot`, `l2_book`
- User data: `portfolio`, `open_orders`, `historical_orders`, `user_fills`
- Trading: `order_status`, `user_rate_limit`, `max_builder_fee`
- Advanced: `delegations`, `referral`, `vault_details`, `sub_accounts`
- Perpetuals: `perpetuals/perp_dexs`, `perpetuals/perpetuals`

### Input Validation
All API methods validate inputs before making requests:
- Ethereum addresses must be 42 chars, start with "0x", hex only
- Coin symbols: 1-10 chars, alphanumeric plus `-` and `_`
- Time intervals: Must be from predefined list (`1m`, `5m`, `1h`, etc.)
- Time ranges: start_time must be less than end_time

### Configuration Options
Environment variables (all optional):
- `HYPERLIQUID_BASE_URL` - API base URL
- `HYPERLIQUID_TIMEOUT_SECS` - Request timeout
- `HYPERLIQUID_MAX_RETRIES` - Retry attempts
- `HYPERLIQUID_ENABLE_COMPRESSION` - Enable gzip
- `HYPERLIQUID_USER_AGENT` - Custom user agent

### Error Recovery
- Automatic retries with exponential backoff for network errors
- Rate limit detection and waiting for HTTP 429 responses
- Connection pooling prevents connection overhead
- Timeout handling with configurable durations

## Development Guidelines

### Adding New API Endpoints
1. Create new module in `src/info/` 
2. Define request/response types with serde derives
3. Add validation for input parameters
4. Use existing common request methods from `src/common.rs`
5. Add comprehensive tests including validation edge cases
6. Update `src/lib.rs` module exports

### Testing New Code
- Add unit tests for validation logic
- Integration tests for actual API calls (mark with `#[ignore]`)
- Test both success and error cases
- Use realistic but non-sensitive test data