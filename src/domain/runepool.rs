use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize)]
pub struct RUNEPoolHistoryQuery {
    pub interval: Option<String>,
    pub count: Option<i64>,
    pub from: Option<i64>,
    pub to: Option<i64>,
}

#[derive(Serialize, FromRow)]
pub struct RUNEPoolHistoryMeta {
    pub start_time: String,
    pub end_time: String,
    pub start_units: String,
    pub start_count: String,
    pub end_units: String,
    pub end_count: String,
    pub next_page: Option<String>,
}

#[derive(Serialize, FromRow, Debug)]
pub struct RUNEPoolHistoryInterval {
    pub start_time: String,
    pub end_time: String,
    pub count: String,
    pub units: String,
}

#[derive(FromRow, Debug)]
pub struct RUNEPoolHistoryIntervalGroup {
    pub record_date: String,
    // pub record_date: NaiveDate,
    pub first_record: DateTime<Utc>,
    pub first_units: i64,
    pub first_count: i64,
    pub last_record: DateTime<Utc>,
    pub last_count: i64,
    pub last_units: i64,
}

#[derive(Serialize)]
pub struct RUNEPoolHistoryResponse {
    pub intervals: Vec<RUNEPoolHistoryInterval>,
    pub meta: RUNEPoolHistoryMeta,
}
