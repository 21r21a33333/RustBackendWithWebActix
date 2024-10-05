
use chrono::Utc;

// use sqlx::types::chrono::Utc;

use actix_web::{web::{Data,Json,Query}, HttpResponse, Responder,post,get,};
use serde::{Deserialize, Serialize};

use sqlx::{types::Decimal, FromRow};
use sqlx::query;
use sqlx::Error;
use sqlx::MySqlPool;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime};
use sqlx::types::time::PrimitiveDateTime;


#[derive(Deserialize)]
struct SwapsQuery {
    interval: Option<String>,
    count: Option<i64>,
    from: Option<i64>,
    to: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
 struct SwapsInterval {
     average_slip: String,
     end_time: String,
     from_trade_average_slip: String,
     from_trade_count: String,
     from_trade_fees: String,
     from_trade_volume: String,
     from_trade_volume_usd: String,
     rune_price_usd: String,
     start_time: String,
     synth_mint_average_slip: String,
     synth_mint_count: String,
     synth_mint_fees: String,
     synth_mint_volume: String,
     synth_mint_volume_usd: String,
     synth_redeem_average_slip: String,
     synth_redeem_count: String,
     synth_redeem_fees: String,
     synth_redeem_volume: String,
     synth_redeem_volume_usd: String,
     to_asset_average_slip: String,
     to_asset_count: String,
     to_asset_fees: String,
     to_asset_volume: String,
     to_asset_volume_usd: String,
     to_rune_average_slip: String,
     to_rune_count: String,
     to_rune_fees: String,
     to_rune_volume: String,
     to_rune_volume_usd: String,
     to_trade_average_slip: String,
     to_trade_count: String,
     to_trade_fees: String,
     to_trade_volume: String,
     to_trade_volume_usd: String,
     total_count: String,
     total_fees: String,
     total_volume: String,
     total_volume_usd: String,
}
#[derive(Debug, Serialize, Deserialize)
]
 struct SwapsMeta {
     average_slip: String,
     end_time: String,
     from_trade_average_slip: String,
     from_trade_count: String,
     from_trade_fees: String,
     from_trade_volume: String,
     from_trade_volume_usd: String,
     rune_price_usd: String,
     start_time: String,
     synth_mint_average_slip: String,
     synth_mint_count: String,
     synth_mint_fees: String,
     synth_mint_volume: String,
     synth_mint_volume_usd: String,
     synth_redeem_average_slip: String,
     synth_redeem_count: String,
     synth_redeem_fees: String,
     synth_redeem_volume: String,
     synth_redeem_volume_usd: String,
     to_asset_average_slip: String,
     to_asset_count: String,
     to_asset_fees: String,
     to_asset_volume: String,
     to_asset_volume_usd: String,
     to_rune_average_slip: String,
     to_rune_count: String,
     to_rune_fees: String,
     to_rune_volume: String,
     to_rune_volume_usd: String,
     to_trade_average_slip: String,
     to_trade_count: String,
     to_trade_fees: String,
     to_trade_volume: String,
     to_trade_volume_usd: String,
     total_count: String,
     total_fees: String,
     total_volume: String,
     total_volume_usd: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct SwapResponse {
    intervals: Vec<SwapsInterval>,               // List of interval structs
    meta: SwapsMeta                              // Meta struct for aggregated data
}




#[get("/history/swaps")]
async fn get_swaps_history(
    pool: Data<MySqlPool>,
    query: Query<SwapsQuery>,
) -> impl Responder {
    let interval = query.interval.clone().unwrap_or_else(|| "day".to_string());
    let count = query.count.unwrap_or(15);

    if count < 1 || count > 400 {
        return HttpResponse::BadRequest().body("Count must be between 1 and 400");
    }

    let to = query.to.unwrap_or_else(|| chrono::Utc::now().timestamp());  // Default to current time
    let from = query.from.unwrap_or_else(|| {
        // Calculate from based on the interval and count if not provided
        match interval.as_str() {
            "hour" => to - Duration::hours(count).num_seconds(),
            "day" => to - Duration::days(count).num_seconds(),
            "week" => to - Duration::weeks(count).num_seconds(),
            "month" => to - Duration::days(30 * count).num_seconds(),
            "quarter" => to - Duration::days(90 * count).num_seconds(),
            "year" => to - Duration::days(365 * count).num_seconds(),
            a => if a.len() ==0 {
                to - Duration::days(count).num_seconds()
            }else{
                -1
            },  // Handle invalid interval
        }
    });
    if from == -1 {
        return HttpResponse::BadRequest().body("Invalid interval");
    }

    let result = fetch_data_for_intervals(&pool, from, to, &interval, count).await;

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": format!("Error: {:?}", e) })),
    }
}

async fn fetch_data_for_intervals(
    pool: &MySqlPool,
    from: i64,
    to: i64,
    interval: &str,
    count: i64,
) -> Result<(), sqlx::Error>{
    Ok(())
}