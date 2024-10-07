use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Decimal};

#[derive(Deserialize)]
pub struct SwapsQuery {
    pub interval: Option<String>,
    pub count: Option<i64>,
    pub from: Option<i64>,
    pub to: Option<i64>,
}

#[derive(Debug)]
pub struct SwapsInterval {
    pub average_slip: Decimal,
    pub end_time: DateTime<Utc>,
    pub from_trade_average_slip: Decimal,
    pub from_trade_count: Decimal,
    pub from_trade_fees: i64,
    pub from_trade_volume: i64,
    pub from_trade_volume_usd: Decimal,
    pub start_time: DateTime<Utc>,
    pub rune_price_usd: Decimal,
    pub synth_mint_average_slip: Decimal,
    pub synth_mint_count: i64,
    pub synth_mint_fees: Decimal,
    pub synth_mint_volume: Decimal,
    pub synth_mint_volume_usd: Decimal,
    pub synth_redeem_average_slip: Decimal,
    pub synth_redeem_count: Decimal,
    pub synth_redeem_fees: Decimal,
    pub synth_redeem_volume: Decimal,
    pub synth_redeem_volume_usd: Decimal,
    pub to_asset_average_slip: Decimal,
    pub to_asset_count: i64,
    pub to_asset_fees: i64,
    pub to_asset_volume: i64,
    pub to_asset_volume_usd: i64,
    pub to_rune_average_slip: Decimal,
    pub to_rune_count: i64,
    pub to_rune_fees: i64,
    pub to_rune_volume: i64,
    pub to_rune_volume_usd: Decimal,
    pub to_trade_average_slip: Decimal,
    pub to_trade_count: i64,
    pub to_trade_fees: i64,
    pub to_trade_volume: i64,
    pub to_trade_volume_usd: Decimal,
    pub total_count: Decimal,
    pub total_fees: Decimal,
    pub total_volume: Decimal,
    pub total_volume_usd: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapsMeta {
    pub average_slip: String,
    pub end_time: String,
    pub from_trade_average_slip: String,
    pub from_trade_count: String,
    pub from_trade_fees: String,
    pub from_trade_volume: String,
    pub from_trade_volume_usd: String,
    pub rune_price_usd: String,
    pub start_time: String,
    pub synth_mint_average_slip: String,
    pub synth_mint_count: String,
    pub synth_mint_fees: String,
    pub synth_mint_volume: String,
    pub synth_mint_volume_usd: String,
    pub synth_redeem_average_slip: String,
    pub synth_redeem_count: String,
    pub synth_redeem_fees: String,
    pub synth_redeem_volume: String,
    pub synth_redeem_volume_usd: String,
    pub to_asset_average_slip: String,
    pub to_asset_count: String,
    pub to_asset_fees: String,
    pub to_asset_volume: String,
    pub to_asset_volume_usd: String,
    pub to_rune_average_slip: String,
    pub to_rune_count: String,
    pub to_rune_fees: String,
    pub to_rune_volume: String,
    pub to_rune_volume_usd: String,
    pub to_trade_average_slip: String,
    pub to_trade_count: String,
    pub to_trade_fees: String,
    pub to_trade_volume: String,
    pub to_trade_volume_usd: String,
    pub total_count: String,
    pub total_fees: String,
    pub total_volume: String,
    pub total_volume_usd: String,
    pub next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapsIntervalResponse {
    pub average_slip: String,
    pub end_time: String,
    pub from_trade_average_slip: String,
    pub from_trade_count: String,
    pub from_trade_fees: String,
    pub from_trade_volume: String,
    pub from_trade_volume_usd: String,
    pub start_time: String,
    pub rune_price_usd: String,
    pub synth_mint_average_slip: String,
    pub synth_mint_count: String,
    pub synth_mint_fees: String,
    pub synth_mint_volume: String,
    pub synth_mint_volume_usd: String,
    pub synth_redeem_average_slip: String,
    pub synth_redeem_count: String,
    pub synth_redeem_fees: String,
    pub synth_redeem_volume: String,
    pub synth_redeem_volume_usd: String,
    pub to_asset_average_slip: String,
    pub to_asset_count: String,
    pub to_asset_fees: String,
    pub to_asset_volume: String,
    pub to_asset_volume_usd: String,
    pub to_rune_average_slip: String,
    pub to_rune_count: String,
    pub to_rune_fees: String,
    pub to_rune_volume: String,
    pub to_rune_volume_usd: String,
    pub to_trade_average_slip: String,
    pub to_trade_count: String,
    pub to_trade_fees: String,
    pub to_trade_volume: String,
    pub to_trade_volume_usd: String,
    pub total_count: String,
    pub total_fees: String,
    pub total_volume: String,
    pub total_volume_usd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapResponse {
    pub intervals: Vec<SwapsIntervalResponse>, // List of interval structs
    pub meta: SwapsMeta,                       // Meta struct for aggregated data
}

#[derive(Debug, FromRow)]
pub struct SwapGroup {
    // Record Date
    pub record_date: String,
    // First Record Fields
    pub first_start_time: DateTime<Utc>,          // TIMESTAMP
    pub first_end_time: DateTime<Utc>,            // TIMESTAMP
    pub first_average_slip: Decimal,              // DECIMAL(20, 10)
    pub first_from_trade_average_slip: Decimal,   // DECIMAL(20, 10)
    pub first_from_trade_count: i64,              // INT
    pub first_from_trade_fees: i64,               // BIGINT
    pub first_from_trade_volume: i64,             // BIGINT
    pub first_from_trade_volume_usd: Decimal,     // DECIMAL(20, 10)
    pub first_rune_price_usd: Decimal,            // DECIMAL(20, 10)
    pub first_synth_mint_average_slip: Decimal,   // DECIMAL(20, 10)
    pub first_synth_mint_count: i64,              // INT
    pub first_synth_mint_fees: i64,               // BIGINT
    pub first_synth_mint_volume: i64,             // BIGINT
    pub first_synth_mint_volume_usd: Decimal,     // DECIMAL(20, 10)
    pub first_synth_redeem_average_slip: Decimal, // DECIMAL(20, 10)
    pub first_synth_redeem_count: i64,            // INT
    pub first_synth_redeem_fees: i64,             // BIGINT
    pub first_synth_redeem_volume: i64,           // BIGINT
    pub first_synth_redeem_volume_usd: Decimal,   // DECIMAL(20, 10)
    pub first_to_asset_average_slip: Decimal,     // DECIMAL(20, 10)
    pub first_to_asset_count: i64,                // INT
    pub first_to_asset_fees: i64,                 // BIGINT
    pub first_to_asset_volume: i64,               // BIGINT
    pub first_to_asset_volume_usd: Decimal,       // DECIMAL(20, 10)
    pub first_to_rune_average_slip: Decimal,      // DECIMAL(20, 10)
    pub first_to_rune_count: i64,                 // INT
    pub first_to_rune_fees: i64,                  // BIGINT
    pub first_to_rune_volume: i64,                // BIGINT
    pub first_to_rune_volume_usd: Decimal,        // DECIMAL(20, 10)
    pub first_to_trade_average_slip: Decimal,     // DECIMAL(20, 10)
    pub first_to_trade_count: i64,                // INT
    pub first_to_trade_fees: i64,                 // BIGINT
    pub first_to_trade_volume: i64,               // BIGINT
    pub first_to_trade_volume_usd: Decimal,       // DECIMAL(20, 10)
    pub first_total_count: i64,                   // INT
    pub first_total_fees: i64,                    // BIGINT
    pub first_total_volume: i64,                  // BIGINT
    pub first_total_volume_usd: Decimal,          // DECIMAL(20, 10)
    // Last Record Fields
    pub last_start_time: DateTime<Utc>,          // TIMESTAMP
    pub last_end_time: DateTime<Utc>,            // TIMESTAMP
    pub last_average_slip: Decimal,              // DECIMAL(20, 10)
    pub last_from_trade_average_slip: Decimal,   // DECIMAL(20, 10)
    pub last_from_trade_count: i64,              // INT
    pub last_from_trade_fees: i64,               // BIGINT
    pub last_from_trade_volume: i64,             // BIGINT
    pub last_from_trade_volume_usd: Decimal,     // DECIMAL(20, 10)
    pub last_rune_price_usd: Decimal,            // DECIMAL(20, 10)
    pub last_synth_mint_average_slip: Decimal,   // DECIMAL(20, 10)
    pub last_synth_mint_count: i64,              // INT
    pub last_synth_mint_fees: i64,               // BIGINT
    pub last_synth_mint_volume: i64,             // BIGINT
    pub last_synth_mint_volume_usd: Decimal,     // DECIMAL(20, 10)
    pub last_synth_redeem_average_slip: Decimal, // DECIMAL(20, 10)
    pub last_synth_redeem_count: i64,            // INT
    pub last_synth_redeem_fees: i64,             // BIGINT
    pub last_synth_redeem_volume: i64,           // BIGINT
    pub last_synth_redeem_volume_usd: Decimal,   // DECIMAL(20, 10)
    pub last_to_asset_average_slip: Decimal,     // DECIMAL(20, 10)
    pub last_to_asset_count: i64,                // INT
    pub last_to_asset_fees: i64,                 // BIGINT
    pub last_to_asset_volume: i64,               // BIGINT
    pub last_to_asset_volume_usd: Decimal,       // DECIMAL(20, 10)
    pub last_to_rune_average_slip: Decimal,      // DECIMAL(20, 10)
    pub last_to_rune_count: i64,                 // INT
    pub last_to_rune_fees: i64,                  // BIGINT
    pub last_to_rune_volume: i64,                // BIGINT
    pub last_to_rune_volume_usd: Decimal,        // DECIMAL(20, 10)
    pub last_to_trade_average_slip: Decimal,     // DECIMAL(20, 10)
    pub last_to_trade_count: i64,                // INT
    pub last_to_trade_fees: i64,                 // BIGINT
    pub last_to_trade_volume: i64,               // BIGINT
    pub last_to_trade_volume_usd: Decimal,       // DECIMAL(20, 10)
    pub last_total_count: i64,                   // INT
    pub last_total_fees: i64,                    // BIGINT
    pub last_total_volume: i64,                  // BIGINT
    pub last_total_volume_usd: Decimal,          // DECIMAL(20, 10)

    pub synth_mint_fees_sum: Decimal,
    pub synth_mint_volume_sum: Decimal,
    pub synth_redeem_volume_usd_sum: Decimal,
    pub synth_redeem_average_slip_avg: Decimal,
    pub synth_redeem_count_sum: Decimal,
    pub synth_redeem_fees_sum: Decimal,
    pub synth_redeem_volume_sum: Decimal,
}
