use crate::config; 
use chrono::Utc;
use config::{database_connection, read_config, update_config};
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, Row};
use std::error::Error;
use std::num::ParseIntError;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Deserialize)]
struct DepthsResponse {
    intervals: Vec<DepthInterval>,
    meta: DepthMeta,
}

#[derive(Debug, Deserialize)]
struct DepthInterval {
    assetDepth: String,
    assetPrice: String,
    assetPriceUSD: String,
    endTime: String,
    liquidityUnits: String,
    luvi: String,
    membersCount: String,
    runeDepth: String,
    startTime: String,
    synthSupply: String,
    synthUnits: String,
    units: String,
}

#[derive(Debug, Deserialize)]
struct DepthMeta {
    endAssetDepth: String,
    endLPUnits: String,
    endMemberCount: String,
    endRuneDepth: String,
    endSynthUnits: String,
    endTime: String,
    startTime: String,
}

async fn fetch_and_store_depth_data(
    url: &str,
    pool: &MySqlPool,
) -> Result<DepthsResponse, Box<dyn Error>> {
    let max_retries = 3;
    let mut retries = 0;

    loop {
        match reqwest::get(url).await {
            Ok(response) => {
                let response: DepthsResponse = response.json().await?;

                // Iterate over intervals and store them in the database
                for interval in &response.intervals {
                    let start_time = interval.startTime.parse::<i64>()?;
                    let end_time = interval.endTime.parse::<i64>()?;
                    let asset_depth = interval.assetDepth.parse::<i64>()?;
                    let rune_depth = interval.runeDepth.parse::<i64>()?;
                    let synth_supply = interval.synthSupply.parse::<i64>()?;
                    let synth_units = interval.synthUnits.parse::<i64>()?;
                    let units = interval.units.parse::<i64>()?;
                    let liquidity_units = interval.liquidityUnits.parse::<i64>()?;
                    let members_count = interval.membersCount.parse::<i32>()?;
                    let asset_price = interval.assetPrice.parse::<f64>()?;
                    let asset_price_usd = interval.assetPriceUSD.parse::<f64>()?;
                    let luvi = interval.luvi.parse::<f64>()?;

                    // Insert data into the BTCDepth table
                    sqlx::query(
                        "INSERT INTO BTCDepth (start_time, end_time, asset_depth, asset_price, asset_price_usd, 
                                               liquidity_units, luvi, members_count, rune_depth, synth_supply, 
                                               synth_units, units)
                         VALUES (?, from_unixtime(?), ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                         ON DUPLICATE KEY UPDATE
                         asset_depth = VALUES(asset_depth),
                         asset_price = VALUES(asset_price),
                         asset_price_usd = VALUES(asset_price_usd),
                         liquidity_units = VALUES(liquidity_units),
                         luvi = VALUES(luvi),
                         members_count = VALUES(members_count),
                         rune_depth = VALUES(rune_depth),
                         synth_supply = VALUES(synth_supply),
                         synth_units = VALUES(synth_units),
                         units = VALUES(units)"
                    )
                    .bind(start_time)
                    .bind(end_time)
                    .bind(asset_depth)
                    .bind(asset_price)
                    .bind(asset_price_usd)
                    .bind(liquidity_units)
                    .bind(luvi)
                    .bind(members_count)
                    .bind(rune_depth)
                    .bind(synth_supply)
                    .bind(synth_units)
                    .bind(units)
                    .execute(pool)
                    .await?;
                }

                println!("Depth data stored successfully!");
                return Ok(response);
            }
            Err(err) => {
                retries += 1;
                eprintln!(
                    "Failed to fetch data (attempt {}): {}. Retrying...",
                    retries, err
                );

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


pub async fn fetch_depth_main() -> Result<(), Box<dyn Error>> {
    let config_path = "depthconfig.json"; // Path to your config file

    // Read the initial configuration
    let mut config = read_config(config_path)?;
    let initial_url = config.api_url.clone(); // Use the URL from the config

    // Establish a database connection
    let pool = database_connection().await?;

    let mut previous_end_time = String::new();
    let mut current_url = initial_url.clone();

    loop {
        // Fetch and store depth data
        let response = fetch_and_store_depth_data(&current_url, &pool).await?;

        // Check if the previous endTime matches the current one
        if previous_end_time == response.meta.endTime {
            println!("Same endTime received. Stopping the loop.");
            break;
        }

        // Check if meta.endTime is greater than the current epoch time
        let current_epoch_time = Utc::now().timestamp();
        let meta_end_time = response
            .meta
            .endTime
            .parse::<i64>()
            .map_err(|e: ParseIntError| format!("Failed to parse meta endTime: {}", e))?;

        if meta_end_time >= current_epoch_time {
            println!("meta.endTime is greater than current epoch time. Stopping the loop.");
            break;
        }

        // Update the previous endTime
        previous_end_time = response.meta.endTime.clone();

        // Update the URL with the new endTime
        current_url = format!(
            "https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=5min&count=25&from={}",
            response.meta.endTime
        );

        // Update the configuration file with the new URL
        update_config(config_path, &current_url)?;

        println!("Fetching next data with endTime: {}", response.meta.endTime);
    }

    Ok(())
}
