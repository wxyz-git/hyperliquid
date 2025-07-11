//! # HyperLiquid Rust API Client
//!
//! A robust, production-ready Rust client library for the HyperLiquid API.
//!
//! ## Features
//!
//! - Complete API coverage with type-safe requests and responses
//! - Input validation for all parameters
//! - Automatic retry logic with exponential backoff
//! - Connection pooling and HTTP keep-alive
//! - Comprehensive error handling
//! - Environment-based configuration
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
//! ## Configuration
//!
//! ```rust
//! use hyperliquid::{client::HyperLiquidClient, config::HyperLiquidConfig};
//! use std::time::Duration;
//!
//! let config = HyperLiquidConfig::new()
//!     .with_timeout(Duration::from_secs(30))
//!     .with_max_retries(3);
//!
//! let client = HyperLiquidClient::with_config(config);
//! ```

pub mod client;
pub mod common;
pub mod config;
pub mod errors;
pub mod info {
    pub mod all_mids;
    pub mod candle_snapshot;
    pub mod delegations;
    pub mod delegator_history;
    pub mod delegator_rewards;
    pub mod delegator_summary;
    pub mod frontend_open_orders;
    pub mod historical_orders;
    pub mod l2_book;
    pub mod max_builder_fee;
    pub mod portfolio;
    pub mod referral;
    pub mod open_orders;
    pub mod order_status;
    pub mod sub_accounts;
    pub mod user_fees;
    pub mod user_fills;
    pub mod user_fills_by_time;
    pub mod user_rate_limit;
    pub mod user_role;
    pub mod user_twap_slice_fills;
    pub mod user_vault_equities;
    pub mod vault_details;
    pub mod perpetuals{
        pub mod perpetuals;
        pub mod perp_dexs;
        }
}