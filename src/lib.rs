//! # HyperLiquid Rust API Client
//! 
//! A robust, production-ready Rust client library for the HyperLiquid API with comprehensive 
//! error handling, input validation, retry logic, and extensive testing.
//! 
//! ## Features
//! 
//! - **Complete API Coverage**: All HyperLiquid API endpoints
//! - **Type Safety**: Fully typed requests and responses with serde
//! - **Input Validation**: Validates Ethereum addresses, coin symbols, intervals, and time ranges
//! - **Retry Logic**: Automatic retries with exponential backoff for failed requests
//! - **Rate Limiting**: Built-in handling for API rate limits (HTTP 429)
//! - **Connection Pooling**: Efficient HTTP connection reuse and keep-alive
//! - **Error Handling**: Comprehensive error types with context
//! - **Testing**: Extensive unit and integration tests
//! 
//! ## Quick Start
//! 
//! ```rust
//! use hyperliquid::client::HyperLiquidClient;
//! 
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = HyperLiquidClient::new();
//!     
//!     // Get all mid prices
//!     let mid_prices = client.get_all_mids().await?;
//!     println!("Mid prices: {:?}", mid_prices);
//!     
//!     Ok(())
//! }
//! ```
//! 
//! ## Modules
//! 
//! - [`client`] - Main HyperLiquid client with connection pooling and retry logic
//! - [`config`] - Configuration management with builder pattern and environment variables
//! - [`errors`] - Error types and input validation functions
//! - [`types`] - Common types shared across API endpoints
//! - [`info`] - API endpoint implementations for all HyperLiquid endpoints

/// Core client functionality with connection pooling and retry logic
pub mod client;

/// Common HTTP request utilities and retry logic implementation
pub mod common;

/// Configuration management with builder pattern and environment variable support
pub mod config;

/// Error handling and input validation
pub mod errors;

/// Common types used across multiple API endpoints
pub mod types;

/// API endpoint implementations for all HyperLiquid functionality
/// 
/// This module contains implementations for all HyperLiquid API endpoints, organized by functionality:
/// 
/// ## Market Data
/// - [`all_mids`] - Get mid prices for all trading pairs
/// - [`candle_snapshot`] - Get OHLCV candle data with time range filtering  
/// - [`l2_book`] - Get order book depth data
/// 
/// ## User Account Data
/// - [`portfolio`] - Get portfolio history and PnL data
/// - [`open_orders`] & [`frontend_open_orders`] - Get active order information
/// - [`historical_orders`] - Get order history with detailed status
/// - [`user_fills`] & [`user_fills_by_time`] - Get trade fill data
/// - [`order_status`] - Get individual order status lookup
/// - [`user_rate_limit`] - Get API rate limit status
/// 
/// ## Trading Infrastructure  
/// - [`user_fees`] - Get comprehensive fee structure and discounts
/// - [`max_builder_fee`] - Get builder fee calculations
/// - [`referral`] - Get referral program data
/// 
/// ## Advanced Features
/// - [`delegations`] & delegation-related endpoints - Staking functionality
/// - [`vault_details`] & [`user_vault_equities`] - Vault/fund management
/// - [`sub_accounts`] - Sub-account management
/// - [`user_twap_slice_fills`] - TWAP order execution data
pub mod info {
    /// Get mid prices for all trading pairs
    pub mod all_mids;
    /// Get OHLCV candle data with time range filtering
    pub mod candle_snapshot;
    /// Get delegation/staking information
    pub mod delegations;
    /// Get delegator history
    pub mod delegator_history;
    /// Get delegator rewards information
    pub mod delegator_rewards;
    /// Get delegator summary
    pub mod delegator_summary;
    /// Get frontend-specific open orders
    pub mod frontend_open_orders;
    /// Get historical orders with detailed status
    pub mod historical_orders;
    /// Get order book depth data
    pub mod l2_book;
    /// Get maximum builder fee calculations
    pub mod max_builder_fee;
    /// Get portfolio history and PnL data
    pub mod portfolio;
    /// Get referral program information
    pub mod referral;
    /// Get active open orders
    pub mod open_orders;
    /// Get individual order status
    pub mod order_status;
    /// Get sub-account information
    pub mod sub_accounts;
    /// Get user fee structure and discounts
    pub mod user_fees;
    /// Get user trade fills
    pub mod user_fills;
    /// Get user trade fills by time range
    pub mod user_fills_by_time;
    /// Get user API rate limit status
    pub mod user_rate_limit;
    /// Get user role information
    pub mod user_role;
    /// Get TWAP order execution data
    pub mod user_twap_slice_fills;
    /// Get user vault equity information
    pub mod user_vault_equities;
    /// Get vault details and information
    pub mod vault_details;
}