mod config;
use chrono::Utc; // We no longer need TimeZone
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, Row};
use std::error::Error;
use std::num::ParseIntError;
use std::time::Duration;
use tokio::time::sleep;
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
    let max_retries = 3;
    let mut retries = 0;

    loop {
        match reqwest::get(url).await {
            Ok(response) => {
                println!("{:?}",response);
                let response: Response = response.json().await?;

                // Store intervals in the database
                for interval in &response.intervals {
                    let start_time = interval.startTime.parse::<i64>()
                        .map_err(|e: ParseIntError| format!("Failed to parse startTime: {}", e))?;
                    let end_time = interval.endTime.parse::<i64>()
                        .map_err(|e: ParseIntError| format!("Failed to parse endTime: {}", e))?;
                    // println!("Storing data for interval: {:?}", interval);
                    // println!("Storing data for interval: {:?}", interval);
                    sqlx::query(
                        "INSERT INTO RUNEPool (count, start_time, end_time, units)
                         VALUES (?, from_unixtime(?), from_unixtime(?), ?)
                         ON DUPLICATE KEY UPDATE
                         count = VALUES(count), units = VALUES(units), end_time = VALUES(end_time)"
                    )
                    .bind(&interval.count)
                    .bind(start_time)
                    .bind(end_time)
                    .bind(&interval.units)
                    .execute(pool)
                    .await?;
                }

                println!("Data stored successfully!");
                return Ok(response);
            }
            Err(err) => {
                retries += 1;
                eprintln!("Failed to fetch data (attempt {}): {}. Retrying...", retries, err);

                if retries >= max_retries {
                    eprintln!("Max retries reached. Exiting with error: {}", err);
                    return Err(Box::new(err));
                }

                // Sleep before retrying
                sleep(Duration::from_secs(2)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let initial_url = "https://midgard.ninerealms.com/v2/history/runepool?interval=5min&count=25&from=1683044400";

    // Establish database connection
    let pool = database_connection().await.map_err(|e| format!("DB connection error: {}", e))?;

    let mut previous_end_time = String::new();
    let mut current_url = initial_url.to_string();

    loop {
        match fetch_and_store_data(&current_url, &pool).await {
            Ok(response) => {
                if previous_end_time == response.meta.endTime {
                    println!("Same endTime received. Stopping the loop.");
                    break;
                }

                let current_epoch_time = Utc::now().timestamp();
                let meta_end_time = response.meta.endTime.parse::<i64>()
                    .map_err(|e: ParseIntError| format!("Failed to parse meta endTime: {}", e))?;
                // println!("response.meta.endTime: {}", response.meta.endTime);
                if meta_end_time >= current_epoch_time {
                    println!("meta.endTime is not greater than current epoch time. Stopping the loop.");
                    break;
                }

                previous_end_time = response.meta.endTime.clone();
                current_url = format!(
                    "https://midgard.ninerealms.com/v2/history/runepool?interval=5min&count=25&from={}",
                    response.meta.endTime
                );

                println!("Fetching next data with endTime: {}", response.meta.endTime);
            }
            Err(err) => {
                eprintln!("Error fetching and storing data: {}", err);
                break;
            }
        }
    }

    Ok(())
}
