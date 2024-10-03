mod config;
use chrono::{Utc, TimeZone}; // Import chrono for time handling
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, Row};
use std::error::Error;
use config::database_connection;

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

async fn fetch_and_store_data(url: &str, pool: &MySqlPool) -> Result<Response, Box<dyn Error>> {
    // Fetch data from the URL
    let response: Response = reqwest::get(url)
        .await?
        .json()
        .await?;

    // Iterate over intervals and store them in the database
    for interval in &response.intervals {
        let start_time = interval.startTime.parse::<i64>()?;
        let end_time = interval.endTime.parse::<i64>()?;
        println!("{:?}", interval);
        sqlx::query(
            "INSERT INTO RUNEPool (count, start_time, end_time, units)
             VALUES (?, from_unixtime(?), from_unixtime(?), ?)
             ON DUPLICATE KEY UPDATE
             count = VALUES(count), units = VALUES(units), end_time = VALUES(end_time)"
        )
        .bind(&interval.count)
        .bind(start_time)  // Parse to i64 for epoch time
        .bind(end_time)
        .bind(&interval.units)
        .execute(pool)
        .await?;
    }
    println!("Data stored successfully!");
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let initial_url = "https://midgard.ninerealms.com/v2/history/runepool?interval=5min&count=25&from=1676436900";

    // Establish a database connection
    let pool = database_connection().await?;

    let mut previous_end_time = String::new();
    let mut current_url = initial_url.to_string();

    loop {
        // Fetch and store data
        let response = fetch_and_store_data(&current_url, &pool).await?;
        // println!("{:?}",response);
        // Check if the previous endTime matches the current one
        if previous_end_time == response.meta.endTime {
            println!("Same endTime received. Stopping the loop.");
            break;
        }

        // Check if meta.endTime is greater than the current epoch time
        let current_epoch_time = Utc::now().timestamp();
        let meta_end_time = response.meta.endTime.parse::<i64>()?;

        if meta_end_time >= current_epoch_time {
            println!("meta.endTime is not greater than current epoch time. Stopping the loop.");
            break;
        }

        // Update the previous endTime
        previous_end_time = response.meta.endTime.clone();

        // Update the URL with the new endTime
        current_url = format!(
            "https://midgard.ninerealms.com/v2/history/runepool?interval=5min&count=25&from={}",
            response.meta.endTime
        );

        println!("Fetching next data with endTime: {}", response.meta.endTime);
    }

    Ok(())
}
