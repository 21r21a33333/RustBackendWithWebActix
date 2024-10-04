use actix_web::{web, HttpResponse, Responder, HttpRequest};
use sqlx::mysql::MySqlPool; // Adjust this import based on your SQLx setup
use chrono::{NaiveDateTime, Duration};
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
struct RUNEPoolHistoryQuery {
    interval: Option<String>,
    count: Option<i64>,
    from: Option<i64>,
    to: Option<i64>,
}

// Define the response structs
#[derive(Serialize)]
struct RUNEPoolHistoryMeta {
    start_time: String,
    end_time: String,
    start_units: String,
    start_count: String,
    end_units: String,
    end_count: String,
}

#[derive(Serialize)]
struct RUNEPoolHistoryInterval {
    start_time: String,
    end_time: String,
    count: String,
    units: String,
}

#[derive(Serialize)]
struct RUNEPoolHistoryResponse {
    intervals: Vec<RUNEPoolHistoryInterval>,
    meta: RUNEPoolHistoryMeta,
}

// Update the handler to accept query parameters
async fn get_runepool_history(
    pool: web::Data<MySqlPool>,
    query: web::Query<RUNEPoolHistoryQuery>,
) -> impl Responder {
    // Extract query parameters
    let interval = query.interval.clone().unwrap_or_default();
    let count = query.count;
    let from = query.from;
    let to = query.to;

    let (start_time, end_time) = match (from, to) {
        (Some(f), Some(t)) => (f, t),
        (Some(f), None) => (f, chrono::Utc::now().timestamp()),
        (None, Some(t)) => (0, t), // Default from to the start of the chain
        (None, None) => (0, chrono::Utc::now().timestamp()), // Default to start of chain until now
    };

    let mut query_str = String::new();
    let mut params: Vec<&(dyn sqlx::Encode<'_, sqlx::MySql> + sqlx::Type<sqlx::MySql>)> = vec![];

    if interval.is_empty() {
        // Without Interval: single From..To search
        query_str.push_str("SELECT * FROM runepool WHERE start_time >= ? AND end_time <= ? LIMIT 400");
        params.push(&start_time);
        params.push(&end_time);
    } else {
        // With Interval
        let duration = match interval.as_str() {
            "5min" => Duration::minutes(5),
            "hour" => Duration::hours(1),
            "day" => Duration::days(1),
            "week" => Duration::weeks(1),
            "month" => Duration::days(30),
            "quarter" => Duration::days(90),
            "year" => Duration::days(365),
            _ => return HttpResponse::BadRequest().body("Invalid interval"),
        };

        query_str.push_str("SELECT * FROM runepool WHERE start_time >= ? AND end_time <= ? GROUP BY FLOOR(start_time / ?) LIMIT ?");
        params.push(&start_time);
        params.push(&end_time);
        params.push(&(duration.num_seconds() as i64));
        
        if let Some(c) = count {
            params.push(&(c));
        } else {
            return HttpResponse::BadRequest().body("Count must be provided if interval is specified");
        }
    }

    // Execute the query
    let records = sqlx::query(&query_str)
        .bind(&params)
        .fetch_all(pool.get_ref())
        .await
        .expect("Database query failed");

    // Process records into intervals and meta
    let mut intervals = vec![];
    for record in records {
        let interval = RUNEPoolHistoryInterval {
            start_time: record.start_time.to_string(),
            end_time: record.end_time.to_string(),
            count: record.count.to_string(),
            units: record.units.to_string(),
        };
        intervals.push(interval);
    }

    let meta = RUNEPoolHistoryMeta {
        start_time: intervals.first().map_or("0".to_string(), |i| i.start_time.clone()),
        end_time: intervals.last().map_or("0".to_string(), |i| i.end_time.clone()),
        start_units: intervals.first().map_or("0".to_string(), |i| i.units.clone()),
        start_count: intervals.first().map_or("0".to_string(), |i| i.count.clone()),
        end_units: intervals.last().map_or("0".to_string(), |i| i.units.clone()),
        end_count: intervals.last().map_or("0".to_string(), |i| i.count.clone()),
    };

    // Prepare response
    let response = RUNEPoolHistoryResponse {
        intervals,
        meta,
    };

    HttpResponse::Ok().json(response)
}
