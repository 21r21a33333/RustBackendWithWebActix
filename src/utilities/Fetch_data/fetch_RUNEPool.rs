// // Your existing database connection function
// pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {
//     let pool = MySqlPool::connect("mysql://root:root@localhost:3306/midgard").await?;
//     Ok(pool)
// }
// mod crate::config;
use crate::config::{database_connection};

use reqwest::Error;
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, Row};

#[derive(Debug, Deserialize)]
struct Response {
    intervals: Vec<Interval>,
    meta: Meta,
}
#[derive(Debug, Deserialize)]
struct Interval {
    count: String,
    endTime: String,
    startTime: String,
    units: String,
}

#[derive(Debug, Deserialize)]
struct Meta {
    endCount: String,
    endTime: String,
    endUnits: String,
    startCount: String,
    startTime: String,
    startUnits: String,
}

async fn fetch_and_store_data(url: &str, pool: &MySqlPool) -> Result<(), Error> {
    // Fetch data from the URL
    let response: Response = reqwest::get(url)
        .await?
        .json()
        .await?;

    // Iterate over intervals and store them in the database
    for interval in response.intervals {
        sqlx::query(
            "INSERT INTO example_data (count, start_time, end_time, units) VALUES (?, from_unixtime(?), from_unixtime(?), ?)"
        )
        .bind(&interval.count)
        .bind(interval.startTime.parse::<i64>().unwrap()) // Parse to i64 for epoch time
        .bind(interval.endTime.parse::<i64>().unwrap())
        .bind(&interval.units)
        .execute(pool)
        .await?;
    }

    println!("Data stored successfully!");
    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://midgard.ninerealms.com/v2/history/runepool?interval=hour&count=5&from=1725347297";
    let pool = database_connection().await?;
    fetch_and_store_data(url, &pool).await?;
    Ok(())
}

