//! HyperLiquid API Client
//!
//! This module provides the main client for interacting with the HyperLiquid API.

use reqwest;
use std::sync::Arc;
use crate::config::HyperLiquidConfig;

/// Main client for the HyperLiquid API
/// 
/// The client handles HTTP connections, retries, and request routing.
/// It's designed to be thread-safe and can be shared across async tasks.
/// 
/// # Examples
/// 
/// ```rust
/// use hyperliquid::client::HyperLiquidClient;
/// 
/// # #[tokio::main]
/// # async fn main() -> anyhow::Result<()> {
/// let client = HyperLiquidClient::new();
/// let mid_prices = client.get_all_mids().await?;
/// # Ok(())
/// # }
/// ```
pub struct HyperLiquidClient {
    pub client: reqwest::Client,
    pub base_url: String,
    pub config: Arc<HyperLiquidConfig>,
}

impl HyperLiquidClient {
    /// Create a new client with default configuration
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use hyperliquid::client::HyperLiquidClient;
    /// 
    /// let client = HyperLiquidClient::new();
    /// ```
    pub fn new() -> Self {
        Self::with_config(HyperLiquidConfig::default())
    }
    
    /// Create a new client with custom configuration
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use hyperliquid::{client::HyperLiquidClient, config::HyperLiquidConfig};
    /// use std::time::Duration;
    /// 
    /// let config = HyperLiquidConfig::new()
    ///     .with_timeout(Duration::from_secs(60))
    ///     .with_max_retries(5);
    /// 
    /// let client = HyperLiquidClient::with_config(config);
    /// ```
    pub fn with_config(config: HyperLiquidConfig) -> Self {
        let mut client_builder = reqwest::Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .pool_max_idle_per_host(config.pool_max_idle_per_host)
            .pool_idle_timeout(config.pool_idle_timeout)
            .tcp_keepalive(config.tcp_keepalive)
            .tcp_nodelay(true);
            
        if config.enable_compression {
            client_builder = client_builder.gzip(true);
        }
        
        let client = client_builder
            .build()
            .expect("Failed to create HTTP client");
            
        Self {
            client,
            base_url: config.base_url.clone(),
            config: Arc::new(config),
        }
    }
    
    /// Create a new client with configuration loaded from environment variables
    /// 
    /// # Environment Variables
    /// 
    /// - `HYPERLIQUID_BASE_URL`: API base URL
    /// - `HYPERLIQUID_TIMEOUT_SECS`: Request timeout in seconds
    /// - `HYPERLIQUID_MAX_RETRIES`: Maximum retry attempts
    /// - `HYPERLIQUID_ENABLE_COMPRESSION`: Enable gzip compression
    /// - `HYPERLIQUID_USER_AGENT`: HTTP User-Agent string
    /// 
    /// # Examples
    /// 
    /// ```bash
    /// export HYPERLIQUID_TIMEOUT_SECS=60
    /// export HYPERLIQUID_MAX_RETRIES=5
    /// ```
    /// 
    /// ```rust
    /// use hyperliquid::client::HyperLiquidClient;
    /// 
    /// let client = HyperLiquidClient::from_env();
    /// ```
    pub fn from_env() -> Self {
        Self::with_config(HyperLiquidConfig::from_env())
    }
}  