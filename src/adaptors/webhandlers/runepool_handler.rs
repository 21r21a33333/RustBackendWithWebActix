use std::{ops::BitAnd, str::FromStr};
use chrono::Utc;

// use sqlx::types::chrono::Utc;

use actix_web::{web::{Data,Json,Query}, HttpResponse, Responder,post,get,};
use serde::{Deserialize, Serialize};

use sqlx::FromRow;
use sqlx::query;
use sqlx::Error;
use sqlx::MySqlPool;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime};
use sqlx::types::time::PrimitiveDateTime;


#[derive(Deserialize)]
struct RUNEPoolHistoryQuery {
    interval: Option<String>,
    count: Option<i64>,
    from: Option<i64>,
    to: Option<i64>,
}
#[derive(Serialize,FromRow)]
struct RUNEPoolHistoryMeta {
    start_time: String,
    end_time: String,
    start_units: String,
    start_count: String,
    end_units: String,
    end_count: String,
}

#[derive(Serialize,FromRow,Debug)]

struct RUNEPoolHistoryInterval {
    start_time: String,
    end_time: String,
    count: String,
    units: String,
}

#[derive(FromRow,Debug)]
struct RUNEPoolHistoryIntervalGroup {

    record_date: String,
    // record_date: NaiveDate,
    first_record: DateTime<Utc>,
    first_units: i64,
    first_count: i64,
    last_record: DateTime<Utc>,
    last_count: i64,
    last_units: i64,
}

#[derive(Serialize)]
struct RUNEPoolHistoryResponse {
    intervals: Vec<RUNEPoolHistoryInterval>,
    meta: RUNEPoolHistoryMeta,
}





#[get("/history/runepool")]
async fn get_runepool_history(
    pool: Data<MySqlPool>,
    query: Query<RUNEPoolHistoryQuery>,
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
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}
async fn fetch_data_for_intervals(
    pool: &MySqlPool,
    from: i64,
    to: i64,
    interval: &str,
    count: i64,
) -> Result<RUNEPoolHistoryResponse, sqlx::Error> {

    // Create the correct SQL PARTITION BY clause based on the interval
    let partition_by_clause = match interval {
        "hour" => "DATE_FORMAT(start_time, '%Y-%m-%d %H')",   // Group by year, month, day, and hour
        "day" => "DATE(start_time)",                          // Group by day
        "week" => "YEARWEEK(start_time)",                     // Group by year and week
        "month" => "DATE_FORMAT(start_time, '%Y-%m')",        // Group by year and month
        "year" => "YEAR(start_time)",                         // Group by year
        _ => "DATE(start_time)",                              // Default to day
    };

    // SQL query with the dynamic partition_by_clause
    let query_str = format!(r#"
    WITH RankedRecords AS (
        SELECT 
            start_time,
            units,
            count,
            end_time,
            {} AS record_date,  -- Use the dynamic partition clause here
            ROW_NUMBER() OVER (PARTITION BY {} ORDER BY start_time) AS rn_first,
            ROW_NUMBER() OVER (PARTITION BY {} ORDER BY start_time DESC) AS rn_last
        FROM runepool
        WHERE start_time IS NOT NULL 
          AND start_time >= FROM_UNIXTIME(?) 
          AND start_time <= FROM_UNIXTIME(?)
    )
    SELECT 
        CAST(record_date AS CHAR) AS record_date,
        MAX(CASE WHEN rn_first = 1 THEN start_time END) AS first_record,
        MAX(CASE WHEN rn_first = 1 THEN units END) AS first_units,
        MAX(CASE WHEN rn_first = 1 THEN count END) AS first_count,
        MAX(CASE WHEN rn_last = 1 THEN end_time END) AS last_record,
        MAX(CASE WHEN rn_last = 1 THEN units END) AS last_units,
        MAX(CASE WHEN rn_last = 1 THEN count END) AS last_count
    FROM RankedRecords
    GROUP BY record_date
    ORDER BY record_date
    LIMIT ?;
    "#, partition_by_clause, partition_by_clause, partition_by_clause);

    let mut intervals = Vec::<RUNEPoolHistoryInterval>::new();

    // Execute the query using `sqlx`
    let records: Vec<RUNEPoolHistoryIntervalGroup> = sqlx::query_as::<_, RUNEPoolHistoryIntervalGroup>(&query_str)
        .bind(from)
        .bind(to)
        .bind(count)
        .fetch_all(pool)
        .await?;

    // Populate the response intervals
    for record in records.iter() {
        intervals.push(RUNEPoolHistoryInterval {
            start_time: record.first_record.to_string(),
            end_time: record.last_record.to_string(),
            count: record.last_count.to_string(),
            units: record.last_units.to_string(),
        });
    }

    // Build the metadata
    let metadata = RUNEPoolHistoryMeta {
        start_time: records[0].first_record.to_string(),
        end_time: records[records.len() - 1].last_record.to_string(),
        start_units: records[0].first_units.to_string(),
        start_count: records[0].first_count.to_string(),
        end_units: records[records.len() - 1].last_units.to_string(),
        end_count: records[records.len() - 1].last_count.to_string(),
    };

    // Build the final response
    let returndata = RUNEPoolHistoryResponse {
        intervals,
        meta: metadata,
    };

    Ok(returndata)
}
