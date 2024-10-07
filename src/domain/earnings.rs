use serde::{Deserialize, Serialize};
use chrono::Datelike;
use chrono::Utc;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime};
use serde_json::json;
use sqlx::query;
use sqlx::types::time::PrimitiveDateTime;
use sqlx::Error;
use sqlx::MySqlPool;
use sqlx::{types::Decimal, FromRow};

#[derive(Debug, FromRow)]
pub struct EarningsResponse {
    pub record_date: String, // Assuming it's a string in 'YYYY-MM-DD' format, otherwise use NaiveDate from chrono
    pub pool: String,        // Optional in case it's null
    pub last_rune_price_usd: Decimal, // Optional in case there is no price
    pub partition_start_time: DateTime<Utc>, // Using chrono for datetime handling
    pub partition_end_time: DateTime<Utc>,
    pub total_block_rewards: Decimal,
    pub total_bonding_earnings: Decimal,
    pub total_earnings: Decimal,
    pub total_liquidity_earnings: Decimal,
    pub total_liquidity_fees: Decimal,
    pub total_asset_liquidity_fees: Decimal,
    pub total_rune_liquidity_fees: Decimal,
    pub total_liquidity_fees_rune: Decimal,
    pub total_saver_earning: Decimal,
    pub total_rewards: Decimal,
}

#[derive(Deserialize)]
pub struct EarningsQuery {
    pub interval: Option<String>,
    pub count: Option<i64>,
    pub from: Option<i64>,
    pub to: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoolData {
    pub pool: String,
    pub asset_liquidity_fees: Decimal,
    pub rune_liquidity_fees: Decimal,
    pub total_liquidity_fees_rune: Decimal,
    pub saver_earning: Decimal,
    pub rewards: Decimal,
    pub earnings: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EarningIntervalData {
    pub start_time: String,
    pub end_time: String,
    pub liquidity_fees: String,
    pub block_rewards: String,
    pub earnings: String,
    pub bonding_earnings: String,
    pub liquidity_earnings: String,
    pub avg_node_count: String,
    pub rune_price_usd: String,
    pub pools: Vec<PoolData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EarningMetaData {
    pub start_time: String,
    pub end_time: String,
    pub liquidity_fees: String,
    pub block_rewards: String,
    pub earnings: String,
    pub bonding_earnings: String,
    pub liquidity_earnings: String,
    pub avg_node_count: String,
    pub rune_price_usd: String,
    pub pools: Vec<PoolData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
    pub meta: EarningMetaData,
    pub intervals: Vec<EarningIntervalData>,
}
