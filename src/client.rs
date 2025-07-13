
//! Main HyperLiquid client with connection pooling and retry logic.
//! 
//! This module contains the core [`HyperLiquidClient`] which provides access to all
//! HyperLiquid API endpoints with built-in features like:
//! 
//! - Automatic retry logic with exponential backoff
//! - Connection pooling and keep-alive
//! - Rate limit handling
//! - Input validation
//! - Comprehensive error handling

use reqwest;
use std::sync::Arc;
use crate::config::HyperLiquidConfig;

/// Main client for interacting with the HyperLiquid API
/// 
/// The client is thread-safe and can be cloned cheaply. It includes:
/// - HTTP connection pooling for optimal performance
/// - Automatic retry logic for transient failures
/// - Built-in rate limiting support
/// - Input validation for all parameters
/// 
/// # Examples
/// 
/// ```rust
/// use hyperliquid::client::HyperLiquidClient;
/// use hyperliquid::config::HyperLiquidConfig;
/// use std::time::Duration;
/// 
/// // Create client with default configuration
/// let client = HyperLiquidClient::new();
/// 
/// // Create client with custom configuration
/// let config = HyperLiquidConfig::new()
///     .with_timeout(Duration::from_secs(30))
///     .with_max_retries(5);
/// let client = HyperLiquidClient::with_config(config);
/// ```
#[derive(Clone)]
pub struct HyperLiquidClient {
    /// HTTP client with connection pooling
    pub client: reqwest::Client,
    /// Base URL for the HyperLiquid API
    pub base_url: String,
    /// Configuration settings (shared across clones)
    pub config: Arc<HyperLiquidConfig>,
}

impl HyperLiquidClient {
    /// Create a new HyperLiquid client with default configuration
    /// 
    /// Uses default configuration including:
    /// - 30 second timeout
    /// - 3 retry attempts 
    /// - Connection pooling enabled
    /// - Gzip compression enabled
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
    
    /// Create a new HyperLiquid client with custom configuration
    /// 
    /// # Arguments
    /// 
    /// * `config` - Custom configuration for the client
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use hyperliquid::client::HyperLiquidClient;
    /// use hyperliquid::config::HyperLiquidConfig;
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

    pub fn from_env() -> Self {
        Self::with_config(HyperLiquidConfig::from_env())
    }
}  