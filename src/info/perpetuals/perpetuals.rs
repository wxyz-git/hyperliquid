use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct MetaRequest {
    #[serde(rename = "type")]
    request_type: String,
}

// ============================================================================================

#[derive(Serialize, Debug)]
struct MetaAndAssetCtxsRequest {
    #[serde(rename = "type")]
    request_type: String,
}
// ============================================================================================

#[derive(Serialize, Debug)]
pub struct ClearingHouseStateRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
}
// ============================================================================================
// ============================================================================================

#[derive(Deserialize, Debug)]
pub struct MetaResponse {
    pub universe: Vec<Perp>,
    #[serde(rename = "marginTables")]
    pub margin_tables: Vec<(u8, MarginTable)>,
}

#[derive(Deserialize, Debug)]
pub struct Perp {
    #[serde(rename = "szDecimals")]
    pub sz_decimals: u8,
    pub name: String,
    #[serde(rename = "maxLeverage")]
    pub max_leverage: u8,
    #[serde(rename = "marginTableId")]
    pub margin_table_id: u8,
    #[serde(rename = "onlyIsolated", skip_serializing_if = "Option::is_none")]
    pub only_isolated: Option<bool>,
    #[serde(rename = "isDelisted", skip_serializing_if = "Option::is_none")]
    pub is_delisted: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct MarginTable {
    pub description: String,
    #[serde(rename = "marginTiers")]
    pub margin_tiers: Vec<MarginTier>,
}

#[derive(Deserialize, Debug)]
pub struct MarginTier {
    #[serde(rename = "lowerBound")]
    pub lower_bound: String, // Using String to preserve exact decimal representation
    #[serde(rename = "maxLeverage")]
    pub max_leverage: u8,
}
// ============================================================================================

#[derive(Debug, Deserialize)]
pub struct  MetaAndAssetCtxsResponse (
    UniverseData,
    Vec<MarketDataItem>,
);

#[derive(Debug, Deserialize)]
pub struct UniverseData {
    pub universe: Vec<AssetInfo>,
    #[serde(rename = "marginTables")]
    pub margin_tables: Vec<(u32, MarginTable)>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetInfo {
    // #[serde(rename = "szDecimals")]
    pub sz_decimals: u8,
    pub name: String,
    // #[serde(rename = "maxLeverage")]
    pub max_leverage: u32,
    // #[serde(rename = "marginTableId")]
    pub margin_table_id: u32,
    // #[serde(rename = "isDelisted", default)]
    pub is_delisted: Option<bool>,
    // #[serde(rename = "onlyIsolated", default)]
    pub only_isolated: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketDataItem {
    pub funding: String,
    #[serde(rename = "openInterest")]
    pub open_interest: String,
    #[serde(rename = "prevDayPx")]
    pub prev_day_px: String,
    #[serde(rename = "dayNtlVlm")]
    pub day_ntl_vlm: String,
    pub premium: Option<String>,
    #[serde(rename = "oraclePx")]
    pub oracle_px: String,
    #[serde(rename = "markPx")]
    pub mark_px: String,
    #[serde(rename = "midPx")]
    pub mid_px: Option<String>,
    #[serde(rename = "impactPxs")]
    pub impact_pxs: Option<Vec<String>>,
    #[serde(rename = "dayBaseVlm")]
    pub day_base_vlm: String,
}

// ============================================================================================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearingHouseStateResponse {
    pub margin_summary: MarginSummary,
    pub cross_margin_summary: CrossMarginSummary,
    pub cross_maintenance_margin_used: String,
    pub withdrawable: String,
    pub asset_positions: Vec<AssetPosition>,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginSummary {
    pub account_value: String,
    pub total_ntl_pos: String,
    pub total_raw_usd: String,
    pub total_margin_used: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossMarginSummary {
    pub account_value: String,
    pub total_ntl_pos: String,
    pub total_raw_usd: String,
    pub total_margin_used: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPosition {
    #[serde(rename = "type")]
    pub position_type: String,
    pub position: Position,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub coin: String,
    pub szi: String,
    pub leverage: Leverage,
    pub entry_px: String,
    pub position_value: String,
    pub unrealized_pnl: String,
    pub return_on_equity: String,
    pub liquidation_px: String,
    pub margin_used: String,
    pub max_leverage: u32,
    pub cum_funding: CumFunding,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leverage {
    #[serde(rename = "type")]
    pub leverage_type: String,
    pub value: u32,
    pub raw_usd: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CumFunding {
    pub all_time: String,
    pub since_open: String,
    pub since_change: String,
}



// ============================================================================================

pub async fn fetch_meta() -> anyhow::Result<MetaResponse> {
    let client = reqwest::Client::new();
    let request = MetaRequest {
        request_type: "meta".to_string(),
    };

    let response = client
        .post("https://api.hyperliquid.xyz/info")
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;
    
    let meta: MetaResponse = response.json().await?;

    Ok(meta)
}


pub async fn fetch_meta_and_asset_ctxs() -> anyhow::Result<MetaAndAssetCtxsResponse> {
    let client = reqwest::Client::new();
    let request = MetaAndAssetCtxsRequest {
        request_type: "metaAndAssetCtxs".to_string(),
    };

    let response = client
        .post("https://api.hyperliquid.xyz/info")
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    let meta_and_asset_ctxs: MetaAndAssetCtxsResponse = response.json().await?;

    Ok(meta_and_asset_ctxs)
}

pub async fn fetch_clearing_house_state(user: &str) -> anyhow::Result<ClearingHouseStateResponse> {
    let client = reqwest::Client::new();
    let request = ClearingHouseStateRequest {
        request_type: "clearinghouseState".to_string(),
        user: user.to_string(),
    };

    let response = client
        .post("https://api.hyperliquid.xyz/info")
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    let clearing_house_state: ClearingHouseStateResponse = response.json().await?;

    Ok(clearing_house_state)
}


// curl -X POST https://api.hyperliquid.xyz/info \
//   -H "Content-Type: application/json" \
//   -d '{"type":"metaAndAssetCtxs"}' | jq > meta.json