use std::time::Duration;

#[derive(Debug, Clone)]
pub struct HyperLiquidConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub max_retries: u32,
    pub enable_compression: bool,
    pub user_agent: String,
    pub pool_max_idle_per_host: usize,
    pub pool_idle_timeout: Duration,
    pub tcp_keepalive: Duration,
}

impl Default for HyperLiquidConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.hyperliquid.xyz".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            enable_compression: true,
            user_agent: format!("hyperliquid-rust-client/{}", env!("CARGO_PKG_VERSION")),
            pool_max_idle_per_host: 10,
            pool_idle_timeout: Duration::from_secs(90),
            tcp_keepalive: Duration::from_secs(60),
        }
    }
}

impl HyperLiquidConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }
    
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }
    
    pub fn with_compression(mut self, enable_compression: bool) -> Self {
        self.enable_compression = enable_compression;
        self
    }
    
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }
    
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        if let Ok(base_url) = std::env::var("HYPERLIQUID_BASE_URL") {
            config.base_url = base_url;
        }
        
        if let Ok(timeout_str) = std::env::var("HYPERLIQUID_TIMEOUT_SECS") {
            if let Ok(timeout_secs) = timeout_str.parse::<u64>() {
                config.timeout = Duration::from_secs(timeout_secs);
            }
        }
        
        if let Ok(retries_str) = std::env::var("HYPERLIQUID_MAX_RETRIES") {
            if let Ok(max_retries) = retries_str.parse::<u32>() {
                config.max_retries = max_retries;
            }
        }
        
        if let Ok(compression_str) = std::env::var("HYPERLIQUID_ENABLE_COMPRESSION") {
            config.enable_compression = compression_str.to_lowercase() == "true";
        }
        
        if let Ok(user_agent) = std::env::var("HYPERLIQUID_USER_AGENT") {
            config.user_agent = user_agent;
        }
        
        config
    }
}