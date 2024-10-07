use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Decimal};

#[derive(Deserialize)]
pub struct DepthAndPriceHistoryQuery {
    pub interval: Option<String>,
    pub count: Option<i64>,
    pub from: Option<i64>,
    pub to: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct DepthAndPriceHistoryMeta {
    pub end_asset_depth: String,
    pub end_lp_units: String,
    pub end_member_count: String,
    pub end_rune_depth: String,
    pub end_synth_units: String,
    pub end_time: String,
    pub luvi_increase: String,
    pub price_shift_loss: String,
    pub start_asset_depth: String,
    pub start_lp_units: String,
    pub start_member_count: String,
    pub start_rune_depth: String,
    pub start_synth_units: String,
    pub start_time: String,
    pub next_page: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DepthAndPriceHistoryInterval {
    pub asset_depth: String,
    pub asset_price: String,
    pub asset_price_usd: String,
    pub end_time: String,
    pub liquidity_units: String,
    pub luvi: String,
    pub members_count: String,
    pub rune_depth: String,
    pub start_time: String,
    pub synth_supply: String,
    pub synth_units: String,
    pub units: String,
}

#[derive(Serialize)]
pub struct DepthAndPriceHistoryResponse {
    pub intervals: Vec<DepthAndPriceHistoryInterval>,
    pub meta: DepthAndPriceHistoryMeta,
}

#[derive(FromRow, Debug)]
pub struct DepthAndPriceHistoryGroup {
    pub record_date: String,
    // pub record_date: NaiveDate,
    pub first_Record: DateTime<Utc>,
    pub last_Record: DateTime<Utc>,
    pub start_Asset_Depth: i64,
    pub start_LP_Units: i64,
    pub start_Member_Count: i64,
    pub start_Rune_Depth: i64,
    pub start_Synth_Units: i64,
    pub start_Luvi: Decimal,
    pub end_Asset_Depth: i64,
    pub end_LP_Units: i64,
    pub end_Member_Count: i64,
    pub end_Rune_Depth: i64,
    pub end_Synth_Units: i64,
    pub end_Luvi: Decimal,
    pub end_assert_price: Decimal,
    pub end_assert_price_usd: Decimal,
    pub start_assert_price: Decimal,
    pub start_assert_price_usd: Decimal,
    pub end_synth_supply: i64,
    pub end_units: i64,
}
