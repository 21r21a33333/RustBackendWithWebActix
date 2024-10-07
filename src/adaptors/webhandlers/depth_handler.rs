use chrono::Utc;

// use sqlx::types::chrono::Utc;

use actix_web::{
    get, post,
    web::{Data, Json, Query},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime};
use sqlx::query;
use sqlx::types::time::PrimitiveDateTime;
use sqlx::Error;
use sqlx::MySqlPool;
use sqlx::{types::Decimal, FromRow};

#[derive(Deserialize)]
struct DepthAndPriceHistoryQuery {
    interval: Option<String>,
    count: Option<i64>,
    from: Option<i64>,
    to: Option<i64>,
}
#[derive(Serialize, Deserialize)]
struct DepthAndPriceHistoryMeta {
    end_asset_depth: String,
    end_lp_units: String,
    end_member_count: String,
    end_rune_depth: String,
    end_synth_units: String,
    end_time: String,
    luvi_increase: String,
    price_shift_loss: String,
    start_asset_depth: String,
    start_lp_units: String,
    start_member_count: String,
    start_rune_depth: String,
    start_synth_units: String,
    start_time: String,
    next_page: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct DepthAndPriceHistoryInterval {
    asset_depth: String,
    asset_price: String,
    asset_price_usd: String,
    end_time: String,
    liquidity_units: String,
    luvi: String,
    members_count: String,
    rune_depth: String,
    start_time: String,
    synth_supply: String,
    synth_units: String,
    units: String,
}

#[derive(Serialize)]
struct DepthAndPriceHistoryResponse {
    intervals: Vec<DepthAndPriceHistoryInterval>,
    meta: DepthAndPriceHistoryMeta,
}

#[derive(FromRow, Debug)]
struct DepthAndPriceHistoryGroup {
    record_date: String,
    // record_date: NaiveDate,
    first_Record: DateTime<Utc>,
    last_Record: DateTime<Utc>,
    start_Asset_Depth: i64,
    start_LP_Units: i64,
    start_Member_Count: i64,
    start_Rune_Depth: i64,
    start_Synth_Units: i64,
    start_Luvi: Decimal,
    end_Asset_Depth: i64,
    end_LP_Units: i64,
    end_Member_Count: i64,
    end_Rune_Depth: i64,
    end_Synth_Units: i64,
    end_Luvi: Decimal,
    end_assert_price: Decimal,
    end_assert_price_usd: Decimal,
    start_assert_price: Decimal,
    start_assert_price_usd: Decimal,
    end_synth_supply: i64,
    end_units: i64,
}

#[get("/depths/BTC.BTC")]
async fn get_depth_and_history(
    pool: Data<MySqlPool>,
    query: Query<DepthAndPriceHistoryQuery>,
) -> impl Responder {
    let interval = query.interval.clone().unwrap_or_else(|| "day".to_string());
    let count = query.count.unwrap_or(15);

    if count < 1 || count > 400 {
        return HttpResponse::BadRequest().body("Count must be between 1 and 400");
    }

    let to = query.to.unwrap_or_else(|| chrono::Utc::now().timestamp()); // Default to current time
    let from = query.from.unwrap_or_else(|| {
        // Calculate from based on the interval and count if not provided
        match interval.as_str() {
            "hour" => to - Duration::hours(count).num_seconds(),
            "day" => to - Duration::days(count).num_seconds(),
            "week" => to - Duration::weeks(count).num_seconds(),
            "month" => to - Duration::days(30 * count).num_seconds(),
            "quarter" => to - Duration::days(90 * count).num_seconds(),
            "year" => to - Duration::days(365 * count).num_seconds(),
            a => {
                if a.len() == 0 {
                    to - Duration::days(count).num_seconds()
                } else {
                    -1
                }
            } // Handle invalid interval
        }
    });
    if from == -1 {
        return HttpResponse::BadRequest().body("Invalid interval");
    }

    let result: Result<DepthAndPriceHistoryResponse, Error> =
        fetch_data_for_intervals(&pool, from, to, &interval, count).await;

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({ "error": format!("Error: {:?}", e) })),
    }
}

async fn fetch_data_for_intervals(
    pool: &MySqlPool,
    from: i64,
    to: i64,
    interval: &str,
    count: i64,
) -> Result<DepthAndPriceHistoryResponse, sqlx::Error> {
    let partition_by_clause = match interval {
        "hour" => "DATE_FORMAT(start_time, '%Y-%m-%d %H')", // Group by year, month, day, and hour
        "day" => "DATE(start_time)",                        // Group by day
        "week" => "YEARWEEK(start_time)",                   // Group by year and week
        "month" => "DATE_FORMAT(start_time, '%Y-%m')",      // Group by year and month
        "year" => "YEAR(start_time)",                       // Group by year
        _ => "DATE(start_time)",                            // Default to day
    };

    let query_str = format!(
        r#"
            WITH RankedRecords AS (
                SELECT
                start_time,
                end_time,
                asset_depth,
                asset_price,
                asset_price_usd,
                liquidity_units, -- corrected from 'liquidty_units'
                luvi,
                members_count,
                rune_depth,
                synth_supply,
                synth_units,
                units,
                {} AS record_date,
                ROW_NUMBER() OVER (PARTITION BY {} ORDER BY start_time) AS rn_first,
                ROW_NUMBER() OVER (PARTITION BY {} ORDER BY start_time DESC) AS rn_last
                FROM btcdepth
                WHERE start_time IS NOT NULL and start_time >= FROM_UNIXTIME(?) AND start_time <=FROM_UNIXTIME(?)
        )
        SELECT
        CAST(record_date AS CHAR) AS record_date,
        MAX(CASE WHEN rn_last = 1 THEN synth_supply END) AS end_synth_supply,
        MAX(CASE WHEN rn_last = 1 THEN units END) AS end_units,
        MAX(CASE WHEN rn_first = 1 THEN start_time END) AS first_Record,
        MAX(CASE WHEN rn_last = 1 THEN DATE_ADD(start_time, INTERVAL 5 MINUTE) END) AS last_Record,
        MAX(CASE WHEN rn_first = 1 THEN asset_depth END) AS start_Asset_Depth,
        MAX(CASE WHEN rn_first = 1 THEN liquidity_units END) AS start_LP_Units, -- corrected spelling
        MAX(CASE WHEN rn_first = 1 THEN members_count END) AS start_Member_Count,
        MAX(CASE WHEN rn_first = 1 THEN rune_depth END) AS start_Rune_Depth,
        MAX(CASE WHEN rn_first = 1 THEN synth_units END) AS start_Synth_Units,
        MAX(CASE WHEN rn_first = 1 THEN luvi END) AS start_Luvi,MAX(CASE WHEN rn_first = 1 THEN asset_price END) AS start_assert_price,
        MAX(CASE WHEN rn_first = 1 THEN asset_price_usd END) AS start_assert_price_usd,

        MAX(CASE WHEN rn_last = 1 THEN asset_depth END) AS end_Asset_Depth,
        MAX(CASE WHEN rn_last = 1 THEN liquidity_units END) AS end_LP_Units, -- corrected spelling
        MAX(CASE WHEN rn_last = 1 THEN members_count END) AS end_Member_Count,
        MAX(CASE WHEN rn_last = 1 THEN rune_depth END) AS end_Rune_Depth,
        MAX(CASE WHEN rn_last = 1 THEN synth_units END) AS end_Synth_Units,
        MAX(CASE WHEN rn_last = 1 THEN luvi END) AS end_Luvi,MAX(CASE WHEN rn_last = 1 THEN asset_price END) AS end_assert_price,
        MAX(CASE WHEN rn_last = 1 THEN asset_price_usd END) AS end_assert_price_usd,
        MAX(CASE WHEN rn_last = 1 THEN synth_supply END) AS end_synth_supply
        FROM RankedRecords
        GROUP BY record_Date
        ORDER BY record_Date
        LIMIT ?;
    "#,
        partition_by_clause, partition_by_clause, partition_by_clause
    );

    let mut intervals = Vec::<DepthAndPriceHistoryInterval>::new();

    let records: Vec<DepthAndPriceHistoryGroup> =
        match sqlx::query_as::<_, DepthAndPriceHistoryGroup>(&query_str)
            .bind(from)
            .bind(to)
            .bind(count)
            .fetch_all(pool)
            .await
        {
            Ok(records) => records,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                return Err(e);
            }
        };

    if records.is_empty() {
        eprintln!("No records found for the given parameters.");
        return Err(sqlx::Error::RowNotFound);
    }

    for record in records.iter() {
        intervals.push(DepthAndPriceHistoryInterval {
            asset_depth: record.end_Asset_Depth.to_string(),
            asset_price: record.end_assert_price.to_string(),
            asset_price_usd: record.end_assert_price_usd.to_string(),
            end_time: record.last_Record.to_string(),
            liquidity_units: record.end_LP_Units.to_string(),
            luvi: record.end_Luvi.to_string(),
            members_count: record.end_Member_Count.to_string(),
            rune_depth: record.end_Rune_Depth.to_string(),
            start_time: record.first_Record.to_string(),
            synth_supply: record.end_synth_supply.to_string(),
            synth_units: record.start_Synth_Units.to_string(),
            units: record.end_units.to_string(),
        });
    }
    let endtime = records.last().unwrap().last_Record.to_string();
    let meta = DepthAndPriceHistoryMeta {
        end_asset_depth: records.last().unwrap().end_Asset_Depth.to_string(),
        end_lp_units: records.last().unwrap().end_LP_Units.to_string(),
        end_member_count: records.last().unwrap().end_Member_Count.to_string(),
        end_rune_depth: records.last().unwrap().end_Rune_Depth.to_string(),
        end_synth_units: records.last().unwrap().end_Synth_Units.to_string(),
        end_time: endtime.clone(),
        luvi_increase: calculate_luvi_increase(&intervals).to_string(),
        price_shift_loss: calculate_price_shift_loss(&intervals).to_string(),
        start_asset_depth: records.first().unwrap().start_Asset_Depth.to_string(),
        start_lp_units: records.first().unwrap().start_LP_Units.to_string(),
        start_member_count: records.first().unwrap().start_Member_Count.to_string(),
        start_rune_depth: records.first().unwrap().start_Rune_Depth.to_string(),
        start_synth_units: records.first().unwrap().start_Synth_Units.to_string(),
        start_time: records.first().unwrap().first_Record.to_string(),
        next_page: Some(
            endtime
                .parse::<DateTime<Utc>>()
                .map(|dt| dt.timestamp().to_string())
                .unwrap_or_else(|_| "Invalid timestamp".to_string()),
        ),
    };
    let response = DepthAndPriceHistoryResponse { intervals, meta };
    return Ok(response);
}

// Helper function to calculate LUVI increase
fn calculate_luvi_increase(records: &[DepthAndPriceHistoryInterval]) -> f64 {
    let start_luvi = records.first().unwrap().luvi.parse::<f64>().unwrap_or(0.0);
    let end_luvi = records.last().unwrap().luvi.parse::<f64>().unwrap_or(0.0);
    if start_luvi == 0.0 {
        return 0.0;
    }
    (end_luvi / start_luvi)
}

// Helper function to calculate price shift loss
fn calculate_price_shift_loss(records: &[DepthAndPriceHistoryInterval]) -> f64 {
    let start_depth = records
        .first()
        .unwrap()
        .asset_depth
        .parse::<f64>()
        .unwrap_or(0.0);
    let end_depth = records
        .last()
        .unwrap()
        .asset_depth
        .parse::<f64>()
        .unwrap_or(0.0);
    if start_depth == 0.0 {
        return 0.0;
    }
    (end_depth / start_depth)
}
