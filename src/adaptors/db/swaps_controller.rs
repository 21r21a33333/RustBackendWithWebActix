use chrono::{DateTime, Utc};
use sqlx::{types::Decimal, MySqlPool};

use crate::{adaptors::getswapquery, domain::{SwapGroup, SwapResponse, SwapsInterval, SwapsIntervalResponse, SwapsMeta}};

pub async fn fetch_data_for_intervals(
    pool: &MySqlPool,
    from: i64,
    to: i64,
    interval: &str,
    count: i64,
) -> Result<SwapResponse, sqlx::Error> {
    println!(
        "interval: {} count {} from {} to {}",
        interval, count, from, to
    );
    let partition_by_clause = match interval {
        "hour" => "DATE_FORMAT(start_time, '%Y-%m-%d %H')", // Group by year, month, day, and hour
        "day" => "DATE(start_time)",                        // Group by day
        "week" => "YEARWEEK(start_time)",                   // Group by year and week
        "month" => "DATE_FORMAT(start_time, '%Y-%m')",      // Group by year and month
        "year" => "YEAR(start_time)",                       // Group by year
        _ => "DATE(start_time)",                            // Default to day
    };

    let query_str = getswapquery(partition_by_clause);
    let mut intervals: Vec<SwapsInterval> = Vec::<SwapsInterval>::new();

    let records: Vec<SwapGroup> = match sqlx::query_as::<_, SwapGroup>(&query_str)
        .bind(from)
        .bind(to)
        .bind(count)
        .fetch_all(pool)
        .await
    {
        Ok(records) => records,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return Err(e);
        }
    };

    if records.is_empty() {
        eprintln!("No records found for the given parameters.");
        return Err(sqlx::Error::RowNotFound);
    }

    for record in records.iter() {
        intervals.push(SwapsInterval {
            average_slip: record.last_average_slip,
            end_time: record.last_end_time,
            from_trade_average_slip: record.first_from_trade_average_slip,
            from_trade_count: record.first_from_trade_average_slip,
            from_trade_fees: record.first_from_trade_fees,
            from_trade_volume: record.first_from_trade_volume,
            from_trade_volume_usd: record.first_from_trade_volume_usd,
            start_time: record.first_start_time,
            rune_price_usd: record.last_rune_price_usd,
            synth_mint_average_slip: record.last_synth_mint_average_slip,
            synth_mint_count: record.last_synth_mint_count,
            synth_mint_fees: record.synth_mint_fees_sum,
            synth_mint_volume: record.synth_mint_volume_sum,
            synth_mint_volume_usd: record.synth_redeem_volume_usd_sum,
            synth_redeem_average_slip: record.synth_redeem_average_slip_avg,
            synth_redeem_count: record.synth_redeem_count_sum,
            synth_redeem_fees: record.synth_redeem_fees_sum,
            synth_redeem_volume: record.synth_redeem_volume_sum,
            synth_redeem_volume_usd: record.synth_redeem_volume_usd_sum,
            to_asset_average_slip: record.last_to_asset_average_slip,
            to_asset_count: record.last_to_asset_count,
            to_asset_fees: record.last_to_asset_fees,
            to_asset_volume: record.last_to_asset_volume,
            to_asset_volume_usd: record.last_to_asset_volume,
            to_rune_average_slip: record.last_to_rune_average_slip,
            to_rune_count: record.last_to_rune_count,
            to_rune_fees: record.last_to_rune_fees,
            to_rune_volume: record.last_to_rune_volume,
            to_rune_volume_usd: record.last_rune_price_usd,
            to_trade_average_slip: record.last_to_trade_average_slip,
            to_trade_count: record.last_to_trade_count,
            to_trade_fees: record.last_to_trade_fees,
            to_trade_volume: record.last_to_trade_volume,
            to_trade_volume_usd: record.last_to_trade_volume_usd,
            total_count: (Decimal::new(record.last_to_asset_count, 0)
                + Decimal::new(record.last_to_rune_count, 0)
                + Decimal::new(record.last_synth_mint_count, 0)
                + record.synth_redeem_count_sum),
            total_fees: (Decimal::new(record.last_to_asset_fees, 0)
                + Decimal::new(record.last_to_rune_fees, 0)
                + Decimal::new(record.last_synth_mint_fees, 0)
                + record.synth_redeem_fees_sum),
            total_volume: (Decimal::new(record.last_to_asset_volume, 0)
                + Decimal::new(record.last_to_rune_volume, 0)
                + record.synth_mint_volume_sum
                + Decimal::new(record.last_synth_redeem_volume, 0)),
            total_volume_usd: (record.last_total_volume_usd),
        });
    }
    let MetaData = compute_meta(&intervals);

    let mut responseinterval: Vec<SwapsIntervalResponse> = Vec::<SwapsIntervalResponse>::new();
    // iterate over intervals and create responseinterval by applying to string method
    for interval in intervals.iter() {
        responseinterval.push(SwapsIntervalResponse {
            average_slip: interval.average_slip.to_string(),
            end_time: interval.end_time.to_string(),
            from_trade_average_slip: interval.from_trade_average_slip.to_string(),
            from_trade_count: interval.from_trade_count.to_string(),
            from_trade_fees: interval.from_trade_fees.to_string(),
            from_trade_volume: interval.from_trade_volume.to_string(),
            from_trade_volume_usd: interval.from_trade_volume_usd.to_string(),
            start_time: interval.start_time.to_string(),
            rune_price_usd: interval.rune_price_usd.to_string(),
            synth_mint_average_slip: interval.synth_mint_average_slip.to_string(),
            synth_mint_count: interval.synth_mint_count.to_string(),
            synth_mint_fees: interval.synth_mint_fees.to_string(),
            synth_mint_volume: interval.synth_mint_volume.to_string(),
            synth_mint_volume_usd: interval.synth_mint_volume_usd.to_string(),
            synth_redeem_average_slip: interval.synth_redeem_average_slip.to_string(),
            synth_redeem_count: interval.synth_redeem_count.to_string(),
            synth_redeem_fees: interval.synth_redeem_fees.to_string(),
            synth_redeem_volume: interval.synth_redeem_volume.to_string(),
            synth_redeem_volume_usd: interval.synth_redeem_volume_usd.to_string(),
            to_asset_average_slip: interval.to_asset_average_slip.to_string(),
            to_asset_count: interval.to_asset_count.to_string(),
            to_asset_fees: interval.to_asset_fees.to_string(),
            to_asset_volume: interval.to_asset_volume.to_string(),
            to_asset_volume_usd: interval.to_asset_volume_usd.to_string(),
            to_rune_average_slip: interval.to_rune_average_slip.to_string(),
            to_rune_count: interval.to_rune_count.to_string(),
            to_rune_fees: interval.to_rune_fees.to_string(),
            to_rune_volume: interval.to_rune_volume.to_string(),
            to_rune_volume_usd: interval.to_rune_volume_usd.to_string(),
            to_trade_average_slip: interval.to_trade_average_slip.to_string(),
            to_trade_count: interval.to_trade_count.to_string(),
            to_trade_fees: interval.to_trade_fees.to_string(),
            to_trade_volume: interval.to_trade_volume.to_string(),
            to_trade_volume_usd: interval.to_trade_volume_usd.to_string(),
            total_count: interval.total_count.to_string(),
            total_fees: interval.total_fees.to_string(),
            total_volume: interval.total_volume.to_string(),
            total_volume_usd: interval.total_volume_usd.to_string(),
        });
    }

    let response = SwapResponse {
        intervals: responseinterval,
        meta: MetaData,
    };

    // todo: create meta informationd and create response

    Ok(response)
}

fn compute_meta(intervals: &Vec<SwapsInterval>) -> SwapsMeta {
    // Initialize accumulators for sums
    let mut total_average_slip = Decimal::ZERO;
    let mut from_trade_average_slip = Decimal::ZERO;
    let mut from_trade_count: Decimal = Decimal::ZERO;
    let mut from_trade_fees = 0;
    let mut from_trade_volume = 0;
    let mut from_trade_volume_usd = Decimal::ZERO;

    let mut synth_mint_average_slip = Decimal::ZERO;
    let mut synth_mint_count = 0;
    let mut synth_mint_fees = Decimal::ZERO;
    let mut synth_mint_volume = Decimal::ZERO;
    let mut synth_mint_volume_usd = Decimal::ZERO;

    let mut synth_redeem_average_slip = Decimal::ZERO;
    let mut synth_redeem_count: Decimal = Decimal::ZERO;
    let mut synth_redeem_fees = Decimal::ZERO;
    let mut synth_redeem_volume = Decimal::ZERO;
    let mut synth_redeem_volume_usd = Decimal::ZERO;

    let mut to_asset_average_slip = Decimal::ZERO;
    let mut to_asset_count = 0;
    let mut to_asset_fees = 0;
    let mut to_asset_volume = 0;
    let mut to_asset_volume_usd = 0;

    let mut to_rune_average_slip = Decimal::ZERO;
    let mut to_rune_count = 0;
    let mut to_rune_fees = 0;
    let mut to_rune_volume = 0;
    let mut to_rune_volume_usd = Decimal::ZERO;

    let mut to_trade_average_slip = Decimal::ZERO;
    let mut to_trade_count = 0;
    let mut to_trade_fees = 0;
    let mut to_trade_volume = 0;
    let mut to_trade_volume_usd = Decimal::ZERO;

    let mut total_count: Decimal = Decimal::ZERO;
    let mut total_fees = Decimal::ZERO;
    let mut total_volume = Decimal::ZERO;
    let mut total_volume_usd = Decimal::ZERO;

    // Loop through intervals to aggregate values
    for interval in intervals {
        total_average_slip += interval.average_slip;
        from_trade_average_slip += interval.from_trade_average_slip;
        from_trade_count += interval.from_trade_count;
        from_trade_fees += interval.from_trade_fees;
        from_trade_volume += interval.from_trade_volume;
        from_trade_volume_usd += interval.from_trade_volume_usd;

        synth_mint_average_slip += interval.synth_mint_average_slip;
        synth_mint_count += interval.synth_mint_count;
        synth_mint_fees += interval.synth_mint_fees;
        synth_mint_volume += interval.synth_mint_volume;
        synth_mint_volume_usd += interval.synth_mint_volume_usd;

        synth_redeem_average_slip += interval.synth_redeem_average_slip;
        synth_redeem_count += interval.synth_redeem_count;
        synth_redeem_fees += interval.synth_redeem_fees;
        synth_redeem_volume += interval.synth_redeem_volume;
        synth_redeem_volume_usd += interval.synth_redeem_volume_usd;

        to_asset_average_slip += interval.to_asset_average_slip;
        to_asset_count += interval.to_asset_count;
        to_asset_fees += interval.to_asset_fees;
        to_asset_volume += interval.to_asset_volume;
        to_asset_volume_usd += interval.to_asset_volume_usd;

        to_rune_average_slip += interval.to_rune_average_slip;
        to_rune_count += interval.to_rune_count;
        to_rune_fees += interval.to_rune_fees;
        to_rune_volume += interval.to_rune_volume;
        to_rune_volume_usd += interval.to_rune_volume_usd;

        to_trade_average_slip += interval.to_trade_average_slip;
        to_trade_count += interval.to_trade_count;
        to_trade_fees += interval.to_trade_fees;
        to_trade_volume += interval.to_trade_volume;
        to_trade_volume_usd += interval.to_trade_volume_usd;

        total_count += interval.total_count;
        total_fees += interval.total_fees;
        total_volume += interval.total_volume;
        total_volume_usd += interval.total_volume_usd;
    }

    let end_time = intervals.last().unwrap().end_time.to_string();
    // Compute averages
    let len = intervals.len() as i64;
    let SwapsMetadata: SwapsMeta = SwapsMeta {
        average_slip: (total_average_slip / Decimal::new(len, 0)).to_string(),
        end_time: end_time.clone(),
        from_trade_average_slip: (from_trade_average_slip / Decimal::new(len, 0)).to_string(),
        from_trade_count: from_trade_count.to_string(),
        from_trade_fees: from_trade_fees.to_string(),
        from_trade_volume: from_trade_volume.to_string(),
        from_trade_volume_usd: (from_trade_volume_usd / Decimal::new(len, 0)).to_string(),
        rune_price_usd: intervals.last().unwrap().rune_price_usd.to_string(),
        start_time: intervals.first().unwrap().start_time.to_string(),
        synth_mint_average_slip: (synth_mint_average_slip / Decimal::new(len, 0)).to_string(),
        synth_mint_count: synth_mint_count.to_string(),
        synth_mint_fees: synth_mint_fees.to_string(),
        synth_mint_volume: synth_mint_volume.to_string(),
        synth_mint_volume_usd: synth_mint_volume_usd.to_string(),
        synth_redeem_average_slip: (synth_redeem_average_slip / Decimal::new(len, 0)).to_string(),
        synth_redeem_count: synth_redeem_count.to_string(),
        synth_redeem_fees: synth_redeem_fees.to_string(),
        synth_redeem_volume: synth_redeem_volume.to_string(),
        synth_redeem_volume_usd: synth_redeem_volume_usd.to_string(),
        to_asset_average_slip: ((to_asset_average_slip / Decimal::new(len, 0)).to_string()),
        to_asset_count: to_asset_count.to_string(),
        to_asset_fees: to_asset_fees.to_string(),
        to_asset_volume: to_asset_volume.to_string(),
        to_asset_volume_usd: to_asset_volume_usd.to_string(),
        to_rune_average_slip: ((to_rune_average_slip / Decimal::new(len, 0)).to_string()),
        to_rune_count: to_rune_count.to_string(),
        to_rune_fees: to_rune_fees.to_string(),
        to_rune_volume: to_rune_volume.to_string(),
        to_rune_volume_usd: to_rune_volume_usd.to_string(),
        to_trade_average_slip: (to_trade_average_slip / Decimal::new(len, 0)).to_string(),
        to_trade_count: to_trade_count.to_string(),
        to_trade_fees: to_trade_fees.to_string(),
        to_trade_volume: to_trade_volume.to_string(),
        to_trade_volume_usd: to_trade_volume_usd.to_string(),
        total_count: total_count.to_string(),
        total_fees: total_fees.to_string(),
        total_volume: total_volume.to_string(),
        total_volume_usd: total_volume_usd.to_string(),
        next: Some(
            end_time
                .parse::<DateTime<Utc>>()
                .map(|dt| dt.timestamp().to_string())
                .unwrap_or_else(|_| "Invalid timestamp".to_string()),
        ),
    };
    SwapsMetadata
}
