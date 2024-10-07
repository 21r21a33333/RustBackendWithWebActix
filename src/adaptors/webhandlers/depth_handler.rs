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

use crate::domain;
use domain::depth::{DepthAndPriceHistoryGroup, DepthAndPriceHistoryInterval, DepthAndPriceHistoryMeta, DepthAndPriceHistoryQuery, DepthAndPriceHistoryResponse};

use crate::adaptors;
use adaptors::db::depth_controller::fetch_data_for_intervals;

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
