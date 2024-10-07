 // Assuming your config module is defined elsewhere
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, Row};
use std::error::Error;
use std::num::ParseIntError;
use std::time::Duration;
use tokio::time::sleep;
use crate::config;
use config::{database_connection, read_config, update_config};

#[derive(Debug, Deserialize)]
struct SwapsResponse {
    intervals: Vec<SwapInterval>,
    meta: SwapMeta,
}

#[derive(Debug, Deserialize)]
struct SwapInterval {
    averageSlip: String,
    endTime: String,
    fromTradeAverageSlip: String,
    fromTradeCount: String,
    fromTradeFees: String,
    fromTradeVolume: String,
    fromTradeVolumeUSD: String,
    runePriceUSD: String,
    startTime: String,
    synthMintAverageSlip: String,
    synthMintCount: String,
    synthMintFees: String,
    synthMintVolume: String,
    synthMintVolumeUSD: String,
    synthRedeemAverageSlip: String,
    synthRedeemCount: String,
    synthRedeemFees: String,
    synthRedeemVolume: String,
    synthRedeemVolumeUSD: String,
    toAssetAverageSlip: String,
    toAssetCount: String,
    toAssetFees: String,
    toAssetVolume: String,
    toAssetVolumeUSD: String,
    toRuneAverageSlip: String,
    toRuneCount: String,
    toRuneFees: String,
    toRuneVolume: String,
    toRuneVolumeUSD: String,
    toTradeAverageSlip: String,
    toTradeCount: String,
    toTradeFees: String,
    toTradeVolume: String,
    toTradeVolumeUSD: String,
    totalCount: String,
    totalFees: String,
    totalVolume: String,
    totalVolumeUSD: String,
}

#[derive(Debug, Deserialize)]
struct SwapMeta {
    averageSlip: String,
    endTime: String,
    fromTradeAverageSlip: String,
    fromTradeCount: String,
    fromTradeFees: String,
    fromTradeVolume: String,
    fromTradeVolumeUSD: String,
    runePriceUSD: String,
    startTime: String,
    synthMintAverageSlip: String,
    synthMintCount: String,
    synthMintFees: String,
    synthMintVolume: String,
    synthMintVolumeUSD: String,
    synthRedeemAverageSlip: String,
    synthRedeemCount: String,
    synthRedeemFees: String,
    synthRedeemVolume: String,
    synthRedeemVolumeUSD: String,
    toAssetAverageSlip: String,
    toAssetCount: String,
    toAssetFees: String,
    toAssetVolume: String,
    toAssetVolumeUSD: String,
    toRuneAverageSlip: String,
    toRuneCount: String,
    toRuneFees: String,
    toRuneVolume: String,
    toRuneVolumeUSD: String,
    toTradeAverageSlip: String,
    toTradeCount: String,
    toTradeFees: String,
    toTradeVolume: String,
    toTradeVolumeUSD: String,
    totalCount: String,
    totalFees: String,
    totalVolume: String,
    totalVolumeUSD: String,
}

async fn fetch_and_store_depth_data(
    url: &str,
    pool: &MySqlPool,
) -> Result<SwapsResponse, Box<dyn Error>> {
    let max_retries = 3;
    let mut retries = 0;

    loop {
        match reqwest::get(url).await {
            Ok(response) => {
                let response: SwapsResponse = response.json().await?;

                // Iterate over intervals and store them in the database
                for interval in &response.intervals {
                    let start_time = interval.startTime.parse::<i64>()?;
                    let end_time = interval.endTime.parse::<i64>()?;
                    let average_slip = interval.averageSlip.parse::<f64>()?;
                    let from_trade_average_slip = interval.fromTradeAverageSlip.parse::<f64>()?;
                    let from_trade_count = interval.fromTradeCount.parse::<i32>()?;
                    let from_trade_fees = interval.fromTradeFees.parse::<i64>()?;
                    let from_trade_volume = interval.fromTradeVolume.parse::<i64>()?;
                    let from_trade_volume_usd = interval.fromTradeVolumeUSD.parse::<f64>()?;
                    let rune_price_usd = interval.runePriceUSD.parse::<f64>()?;
                    let synth_mint_average_slip = interval.synthMintAverageSlip.parse::<f64>()?;
                    let synth_mint_count = interval.synthMintCount.parse::<i32>()?;
                    let synth_mint_fees = interval.synthMintFees.parse::<i64>()?;
                    let synth_mint_volume = interval.synthMintVolume.parse::<i64>()?;
                    let synth_mint_volume_usd = interval.synthMintVolumeUSD.parse::<f64>()?;
                    let synth_redeem_average_slip =
                        interval.synthRedeemAverageSlip.parse::<f64>()?;
                    let synth_redeem_count = interval.synthRedeemCount.parse::<i32>()?;
                    let synth_redeem_fees = interval.synthRedeemFees.parse::<i64>()?;
                    let synth_redeem_volume = interval.synthRedeemVolume.parse::<i64>()?;
                    let synth_redeem_volume_usd = interval.synthRedeemVolumeUSD.parse::<f64>()?;
                    let to_asset_average_slip = interval.toAssetAverageSlip.parse::<f64>()?;
                    let to_asset_count = interval.toAssetCount.parse::<i32>()?;
                    let to_asset_fees = interval.toAssetFees.parse::<i64>()?;
                    let to_asset_volume = interval.toAssetVolume.parse::<i64>()?;
                    let to_asset_volume_usd = interval.toAssetVolumeUSD.parse::<f64>()?;
                    let to_rune_average_slip = interval.toRuneAverageSlip.parse::<f64>()?;
                    let to_rune_count = interval.toRuneCount.parse::<i32>()?;
                    let to_rune_fees = interval.toRuneFees.parse::<i64>()?;
                    let to_rune_volume = interval.toRuneVolume.parse::<i64>()?;
                    let to_rune_volume_usd = interval.toRuneVolumeUSD.parse::<f64>()?;
                    let to_trade_average_slip = interval.toTradeAverageSlip.parse::<f64>()?;
                    let to_trade_count = interval.toTradeCount.parse::<i32>()?;
                    let to_trade_fees = interval.toTradeFees.parse::<i64>()?;
                    let to_trade_volume = interval.toTradeVolume.parse::<i64>()?;
                    let to_trade_volume_usd = interval.toTradeVolumeUSD.parse::<f64>()?;
                    let total_count = interval.totalCount.parse::<i32>()?;
                    let total_fees = interval.totalFees.parse::<i64>()?;
                    let total_volume = interval.totalVolume.parse::<i64>()?;
                    let total_volume_usd = interval.totalVolumeUSD.parse::<f64>()?;

                    sqlx::query(
                        r#"
                        INSERT INTO Swaps (
                            start_time, end_time, average_slip,
                            from_trade_average_slip, from_trade_count,
                            from_trade_fees, from_trade_volume, from_trade_volume_usd,
                            rune_price_usd, synth_mint_average_slip, synth_mint_count,
                            synth_mint_fees, synth_mint_volume, synth_mint_volume_usd,
                            synth_redeem_average_slip, synth_redeem_count, synth_redeem_fees,
                            synth_redeem_volume, synth_redeem_volume_usd, to_asset_average_slip,
                            to_asset_count, to_asset_fees, to_asset_volume, to_asset_volume_usd,
                            to_rune_average_slip, to_rune_count, to_rune_fees,
                            to_rune_volume, to_rune_volume_usd, to_trade_average_slip,
                            to_trade_count, to_trade_fees, to_trade_volume, to_trade_volume_usd,
                            total_count, total_fees, total_volume, total_volume_usd
                        ) VALUES (
                            FROM_UNIXTIME(?), FROM_UNIXTIME(?), ?,
                            ?, ?, ?, ?, ?,
                            ?, ?, ?, ?, ?,
                            ?, ?, ?, ?, ?,
                            ?, ?, ?, ?, ?,
                            ?, ?, ?, ?, ?,
                            ?, ?, ?, ?, ?,
                            ?, ?, ?, ?, ?
                        )
                        ON DUPLICATE KEY UPDATE
                            average_slip = VALUES(average_slip),
                            from_trade_average_slip = VALUES(from_trade_average_slip),
                            from_trade_count = VALUES(from_trade_count),
                            from_trade_fees = VALUES(from_trade_fees),
                            from_trade_volume = VALUES(from_trade_volume),
                            from_trade_volume_usd = VALUES(from_trade_volume_usd),
                            rune_price_usd = VALUES(rune_price_usd),
                            synth_mint_average_slip = VALUES(synth_mint_average_slip),
                            synth_mint_count = VALUES(synth_mint_count),
                            synth_mint_fees = VALUES(synth_mint_fees),
                            synth_mint_volume = VALUES(synth_mint_volume),
                            synth_mint_volume_usd = VALUES(synth_mint_volume_usd),
                            synth_redeem_average_slip = VALUES(synth_redeem_average_slip),
                            synth_redeem_count = VALUES(synth_redeem_count),
                            synth_redeem_fees = VALUES(synth_redeem_fees),
                            synth_redeem_volume = VALUES(synth_redeem_volume),
                            synth_redeem_volume_usd = VALUES(synth_redeem_volume_usd),
                            to_asset_average_slip = VALUES(to_asset_average_slip),
                            to_asset_count = VALUES(to_asset_count),
                            to_asset_fees = VALUES(to_asset_fees),
                            to_asset_volume = VALUES(to_asset_volume),
                            to_asset_volume_usd = VALUES(to_asset_volume_usd),
                            to_rune_average_slip = VALUES(to_rune_average_slip),
                            to_rune_count = VALUES(to_rune_count),
                            to_rune_fees = VALUES(to_rune_fees),
                            to_rune_volume = VALUES(to_rune_volume),
                            to_rune_volume_usd = VALUES(to_rune_volume_usd),
                            to_trade_average_slip = VALUES(to_trade_average_slip),
                            to_trade_count = VALUES(to_trade_count),
                            to_trade_fees = VALUES(to_trade_fees),
                            to_trade_volume = VALUES(to_trade_volume),
                            to_trade_volume_usd = VALUES(to_trade_volume_usd),
                            total_count = VALUES(total_count),
                            total_fees = VALUES(total_fees),
                            total_volume = VALUES(total_volume),
                            total_volume_usd = VALUES(total_volume_usd)
                    "#,
                    )
                    .bind(start_time)
                    .bind(end_time)
                    .bind(average_slip)
                    .bind(from_trade_average_slip)
                    .bind(from_trade_count)
                    .bind(from_trade_fees)
                    .bind(from_trade_volume)
                    .bind(from_trade_volume_usd)
                    .bind(rune_price_usd)
                    .bind(synth_mint_average_slip)
                    .bind(synth_mint_count)
                    .bind(synth_mint_fees)
                    .bind(synth_mint_volume)
                    .bind(synth_mint_volume_usd)
                    .bind(synth_redeem_average_slip)
                    .bind(synth_redeem_count)
                    .bind(synth_redeem_fees)
                    .bind(synth_redeem_volume)
                    .bind(synth_redeem_volume_usd)
                    .bind(to_asset_average_slip)
                    .bind(to_asset_count)
                    .bind(to_asset_fees)
                    .bind(to_asset_volume)
                    .bind(to_asset_volume_usd)
                    .bind(to_rune_average_slip)
                    .bind(to_rune_count)
                    .bind(to_rune_fees)
                    .bind(to_rune_volume)
                    .bind(to_rune_volume_usd)
                    .bind(to_trade_average_slip)
                    .bind(to_trade_count)
                    .bind(to_trade_fees)
                    .bind(to_trade_volume)
                    .bind(to_trade_volume_usd)
                    .bind(total_count)
                    .bind(total_fees)
                    .bind(total_volume)
                    .bind(total_volume_usd)
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


pub async fn fetch_swaps_main() -> Result<(), Box<dyn Error>> {
    let config_path = "swapconfig.json"; // Path to your config file

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
            "https://midgard.ninerealms.com/v2/history/swaps?interval=5min&count=25&from={}",
            response.meta.endTime
        );

        // Update the configuration file with the new URL
        update_config(config_path, &current_url)?;

        println!("Fetching next data with endTime: {}", response.meta.endTime);
    }

    Ok(())
}
