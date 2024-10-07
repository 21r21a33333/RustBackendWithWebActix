use  crate::config; // Assuming your config module is defined elsewhere
use chrono::Utc;
use config::{database_connection, read_config, update_config};
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, Row};
use std::error::Error;
use std::num::ParseIntError;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Deserialize)]
struct EarningsResponse {
    intervals: Vec<EarningsInterval>,
    meta: EarningsMeta,
}

#[derive(Debug, Deserialize)]
struct EarningsInterval {
    avgNodeCount: String,
    blockRewards: String,
    bondingEarnings: String,
    earnings: String,
    endTime: String,
    liquidityEarnings: String,
    liquidityFees: String,
    pools: Vec<EarningsPool>,
    runePriceUSD: String,
    startTime: String,
}

#[derive(Debug, Deserialize)]
struct EarningsPool {
    assetLiquidityFees: String,
    earnings: String,
    pool: String,
    rewards: String,
    runeLiquidityFees: String,
    saverEarning: String,
    totalLiquidityFeesRune: String,
}

#[derive(Debug, Deserialize)]
struct EarningsMeta {
    avgNodeCount: String,
    blockRewards: String,
    bondingEarnings: String,
    earnings: String,
    endTime: String,
    liquidityEarnings: String,
    liquidityFees: String,
    pools: Vec<EarningsPool>,
    runePriceUSD: String,
    startTime: String,
}

async fn fetch_and_store_earnings_data(
    url: &str,
    pool: &MySqlPool,
) -> Result<EarningsResponse, Box<dyn Error>> {
    let max_retries = 3;
    let mut retries = 0;

    loop {
        match reqwest::get(url).await {
            Ok(response) => {
                let response: EarningsResponse = response.json().await?;

                // Iterate over intervals and store them in the database
                for interval in &response.intervals {
                    let start_time = interval.startTime.parse::<i64>()?;
                    let end_time = interval.endTime.parse::<i64>()?;
                    let avg_node_count = interval.avgNodeCount.parse::<f64>()?;
                    let block_rewards = interval.blockRewards.parse::<i64>()?;
                    let bonding_earnings = interval.bondingEarnings.parse::<i64>()?;
                    let earnings = interval.earnings.parse::<i64>()?;
                    let liquidity_earnings = interval.liquidityEarnings.parse::<i64>()?;
                    let liquidity_fees = interval.liquidityFees.parse::<i64>()?;
                    let rune_price_usd = interval.runePriceUSD.parse::<f64>()?;

                    // Insert into Earnings table
                    sqlx::query(
                        r#"
                        INSERT INTO Earnings (
                            start_time, end_time, avg_node_count,
                            block_rewards, bonding_earnings, earnings,
                            liquidity_earnings, liquidity_fees, rune_price_usd
                        ) VALUES (
                            FROM_UNIXTIME(?), FROM_UNIXTIME(?), ?,
                            ?, ?, ?, ?, ?, ?
                        )
                        ON DUPLICATE KEY UPDATE
                            avg_node_count = VALUES(avg_node_count),
                            block_rewards = VALUES(block_rewards),
                            bonding_earnings = VALUES(bonding_earnings),
                            earnings = VALUES(earnings),
                            liquidity_earnings = VALUES(liquidity_earnings),
                            liquidity_fees = VALUES(liquidity_fees),
                            rune_price_usd = VALUES(rune_price_usd)
                        "#,
                    )
                    .bind(start_time)
                    .bind(end_time)
                    .bind(avg_node_count)
                    .bind(block_rewards)
                    .bind(bonding_earnings)
                    .bind(earnings)
                    .bind(liquidity_earnings)
                    .bind(liquidity_fees)
                    .bind(rune_price_usd)
                    .execute(pool)
                    .await?;

                    // Fetch the `id` from the `Earnings` table where start_time equals the inserted value
                    let earnings_id = sqlx::query(
                        r#"
                        SELECT id FROM Earnings WHERE start_time = FROM_UNIXTIME(?)
                        "#,
                    )
                    .bind(start_time)
                    .fetch_one(pool)
                    .await?
                    .get::<i32, _>(0); // `id` field from Earnings table (INT)

                    // Now insert the pool data into EarningsPools table for each pool
                    for pool_data in &interval.pools {
                        let asset_liquidity_fees = pool_data.assetLiquidityFees.parse::<i64>()?;
                        let pool_earnings = pool_data.earnings.parse::<i64>()?;
                        let rewards = pool_data.rewards.parse::<i64>()?;
                        let rune_liquidity_fees = pool_data.runeLiquidityFees.parse::<i64>()?;
                        let saver_earning = pool_data.saverEarning.parse::<i64>()?;
                        let total_liquidity_fees_rune =
                            pool_data.totalLiquidityFeesRune.parse::<i64>()?;

                        // Insert data into the EarningsPools table
                        sqlx::query(
                            r#"
                                INSERT INTO EarningsPools (
                                    earnings_id, pool, asset_liquidity_fees,
                                    earnings, rewards, rune_liquidity_fees,
                                    saver_earning, total_liquidity_fees_rune
                                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                            "#,
                        )
                        .bind(earnings_id)
                        .bind(&pool_data.pool) // Use the pool field from pool_data
                        .bind(asset_liquidity_fees)
                        .bind(pool_earnings)
                        .bind(rewards)
                        .bind(rune_liquidity_fees)
                        .bind(saver_earning)
                        .bind(total_liquidity_fees_rune)
                        .execute(pool) // Execute with the connection pool
                        .await?;
                    }
                }

                return Ok(response);
            }
            Err(_) if retries < max_retries => {
                retries += 1;
                sleep(Duration::from_secs(2)).await;
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
}


pub async fn fetch_earnings_main() -> Result<(), Box<dyn Error>> {
    let config_path = "earningsconfig.json"; // Path to your config file

    // Read the initial configuration
    let config = read_config(config_path)?;
    let initial_url = config.api_url.clone(); // Use the URL from the config

    // Establish a database connection
    let pool = database_connection().await?;

    let mut previous_end_time = String::new();
    let mut current_url = initial_url.clone();

    loop {
        // Fetch and store depth data
        let response = fetch_and_store_earnings_data(&current_url, &pool).await?;

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
        println!("Current epoch time: {} == meta end time ={}", current_epoch_time,meta_end_time);

        if meta_end_time >= current_epoch_time {
            println!("meta.endTime is greater than current epoch time. Stopping the loop.");
            break;
        }

        // Update the previous endTime
        previous_end_time = response.meta.endTime.clone();

        // Update the URL with the new endTime
        current_url = format!(
            "https://midgard.ninerealms.com/v2/history/earnings?interval=hour&count=25&from={}",
            response.meta.endTime
        );

        // Update the configuration file with the new URL
        update_config(config_path, &current_url)?;

        println!("Fetching next data with endTime: {}", response.meta.endTime);
    }

    Ok(())
}
