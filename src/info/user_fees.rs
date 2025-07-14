use rust_decimal::Decimal;
use serde::Deserialize;

use crate::{client::HyperLiquidClient, errors::validate_ethereum_address};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFeesResponse {
    pub daily_user_vlm: Vec<DailyUserVlm>,
    pub fee_schedule: FeeSchedule,
    #[serde(with = "rust_decimal::serde::str")]
    pub user_cross_rate: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub user_add_rate: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub user_spot_cross_rate: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub user_spot_add_rate: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub active_referral_discount: Decimal,
    pub trial: Option<String>,
    #[serde(with = "rust_decimal::serde::str")]
    pub fee_trial_reward: Decimal,
    pub next_trial_available_timestamp: Option<u64>,
    pub staking_link: Option<String>,
    pub active_staking_discount: Option<ActiveStakingDiscount>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyUserVlm {
    pub date: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub user_cross: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub user_add: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub exchange: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeSchedule {
    #[serde(with = "rust_decimal::serde::str")]
    pub cross: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub add: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub spot_cross: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub spot_add: Decimal,
    pub tiers: Tiers,
    #[serde(with = "rust_decimal::serde::str")]
    pub referral_discount: Decimal,
    pub staking_discount_tiers: Vec<StakingDiscountTier>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tiers {
    pub vip: Vec<VipTier>,
    pub mm: Vec<MmTier>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VipTier {
    #[serde(with = "rust_decimal::serde::str")]
    pub ntl_cutoff: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub cross: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub add: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub spot_cross: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub spot_add: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmTier {
    #[serde(with = "rust_decimal::serde::str")]
    pub maker_fraction_cutoff: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub add: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakingDiscountTier {
    #[serde(with = "rust_decimal::serde::str")]
    pub bps_of_max_supply: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub discount: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveStakingDiscount {
    pub bps_of_max_supply: String,
    pub discount: String,
}

impl HyperLiquidClient {
    pub async fn get_user_fees(&self, user: &str) -> anyhow::Result<UserFeesResponse> {
        validate_ethereum_address(user)?;
        self.make_user_request("userFees", user).await
    }
}