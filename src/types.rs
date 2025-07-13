//! Common types used across multiple API endpoints.

use rust_decimal::Decimal;
use serde::Deserialize;

/// Represents an order in the HyperLiquid system.
/// Used by order status, historical orders, and open orders endpoints.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// The trading pair symbol (e.g., "BTC", "ETH")
    pub coin: String,
    
    /// Order side: "A" for ask/sell, "B" for bid/buy
    pub side: String,
    
    /// Limit price for the order
    #[serde(rename = "limitPx")]
    #[serde(with = "rust_decimal::serde::str")]
    pub limit_px: Decimal,
    
    /// Order size
    #[serde(with = "rust_decimal::serde::str")]
    pub sz: Decimal,
    
    /// Unique order ID
    pub oid: u64,
    
    /// Order timestamp (milliseconds since Unix epoch)
    pub timestamp: u64,
    
    /// Trigger condition for conditional orders
    #[serde(rename = "triggerCondition")]
    pub trigger_condition: String,
    
    /// Whether this is a trigger order
    #[serde(rename = "isTrigger")]
    pub is_trigger: bool,
    
    /// Trigger price for conditional orders
    #[serde(rename = "triggerPx")]
    #[serde(with = "rust_decimal::serde::str")]
    pub trigger_px: Decimal,
    
    /// Child orders (flexible type to handle different API responses)
    pub children: Vec<serde_json::Value>,
    
    /// Whether this is a position take-profit/stop-loss order
    #[serde(rename = "isPositionTpsl")]
    pub is_position_tpsl: bool,
    
    /// Whether this is a reduce-only order
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    
    /// Order type (e.g., "limit", "market")
    #[serde(rename = "orderType")]
    pub order_type: String,
    
    /// Original order size
    #[serde(rename = "origSz")]
    #[serde(with = "rust_decimal::serde::str")]
    pub orig_sz: Decimal,
    
    /// Time in force (e.g., "Gtc", "Ioc")
    #[serde(default)]
    pub tif: Option<String>,
    
    /// Client order ID (optional)
    #[serde(default)]
    pub cloid: Option<String>,
}