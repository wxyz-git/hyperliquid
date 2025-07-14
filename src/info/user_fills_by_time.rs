use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::client::HyperLiquidClient;
use crate::errors::{validate_ethereum_address, validate_time_range};

#[derive(Serialize)]
struct UserFillsByTimeRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
    #[serde(rename = "startTime")]
    start_time: u64,
    #[serde(rename = "endTime")]
    end_time: Option<u64>,
    #[serde(rename = "aggregatedByTime")]
    aggregated_by_time: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFillsByTimeResponse {
    pub coin: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub px: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub sz: Decimal,
    pub side: String,
    pub time: u64,
    #[serde(with = "rust_decimal::serde::str")]
    pub start_position: Decimal,
    pub dir: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub closed_pnl: Decimal,
    pub hash: String,
    pub oid: u64,
    pub crossed: bool,
    #[serde(with = "rust_decimal::serde::str")]
    pub fee: Decimal,
    pub tid: u64,
    pub fee_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidation: Option<Liquidation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Liquidation {
    pub liquidated_user: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub mark_px: Decimal,
    pub method: String,
}

impl HyperLiquidClient {
    pub async fn get_user_fills_by_time(&self, user: &str, start_time: u64, end_time: Option<u64>, aggregated_by_time: bool) -> anyhow::Result<Vec<UserFillsByTimeResponse>> {
        validate_ethereum_address(user)?;
        if let Some(end_time) = end_time {
            validate_time_range(start_time, end_time)?;
        }
        
        let request_body = UserFillsByTimeRequest {
            request_type: "userFillsByTime".to_string(),
            user: user.to_string(),
            start_time: start_time,
            end_time: end_time,
            aggregated_by_time: aggregated_by_time,
        };

        self.make_custom_request(&request_body).await
    }
}