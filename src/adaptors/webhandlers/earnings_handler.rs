use std::collections::HashMap;
use std::str::FromStr;

use chrono::Datelike;
use chrono::Utc;

// use sqlx::types::chrono::Utc;

use actix_web::{
    get, post,
    web::{Data, Json, Query},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime};
use serde_json::json;
use sqlx::query;
use sqlx::types::time::PrimitiveDateTime;
use sqlx::Error;
use sqlx::MySqlPool;
use sqlx::{types::Decimal, FromRow};

#[derive(Debug, FromRow)]
struct EarningsResponse {
    record_date: String, // Assuming it's a string in 'YYYY-MM-DD' format, otherwise use NaiveDate from chrono
    pool: String,        // Optional in case it's null
    last_rune_price_usd: Decimal, // Optional in case there is no price
    partition_start_time: DateTime<Utc>, // Using chrono for datetime handling
    partition_end_time: DateTime<Utc>,
    total_block_rewards: Decimal,
    total_bonding_earnings: Decimal,
    total_earnings: Decimal,
    total_liquidity_earnings: Decimal,
    total_liquidity_fees: Decimal,
    total_asset_liquidity_fees: Decimal,
    total_rune_liquidity_fees: Decimal,
    total_liquidity_fees_rune: Decimal,
    total_saver_earning: Decimal,
    total_rewards: Decimal,
}

#[derive(Deserialize)]
struct EarningsQuery {
    interval: Option<String>,
    count: Option<i64>,
    from: Option<i64>,
    to: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PoolData {
    pool: String,
    asset_liquidity_fees: Decimal,
    rune_liquidity_fees: Decimal,
    total_liquidity_fees_rune: Decimal,
    saver_earning: Decimal,
    rewards: Decimal,
    earnings: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
struct EarningIntervalData {
    start_time: String,
    end_time: String,
    liquidity_fees: String,
    block_rewards: String,
    earnings: String,
    bonding_earnings: String,
    liquidity_earnings: String,
    avg_node_count: String,
    rune_price_usd: String,
    pools: Vec<PoolData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EarningMetaData {
    start_time: String,
    end_time: String,
    liquidity_fees: String,
    block_rewards: String,
    earnings: String,
    bonding_earnings: String,
    liquidity_earnings: String,
    avg_node_count: String,
    rune_price_usd: String,
    pools: Vec<PoolData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseData {
    meta: EarningMetaData,
    intervals: Vec<EarningIntervalData>,
}

#[get("/history/earnings")]
async fn get_earnings(pool: Data<MySqlPool>, query: Query<EarningsQuery>) -> impl Responder {
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

    let result = fetch_data_for_intervals(&pool, from, to, &interval, count).await;

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({ "error": format!("Error: {:?}", e) })),
    }
}

async fn fetch_data_for_intervals(
    pool: &MySqlPool,
    from: i64,
    to: i64,
    interval: &str,
    count: i64,
) -> Result<ResponseData, sqlx::Error> {
    println!(
        "interval: {:?} from {:?} to {:?} count {:?}",
        interval, from, to, count
    );
    let partition_by_clause = match interval {
        "hour" => "DATE_FORMAT(start_time, '%Y-%m-%d %H')", // Group by year, month, day, and hour
        "day" => "DATE(start_time)",                        // Group by day
        "week" => "YEARWEEK(start_time)",                   // Group by year and week
        "month" => "DATE_FORMAT(start_time, '%Y-%m')",      // Group by year and month
        "year" => "YEAR(start_time)",                       // Group by year
        _ => "DATE(start_time)",                            // Default to day
    };

    let query_str = format!(
        r#"
    WITH earninginterval AS (
        SELECT 
            *
        FROM earnings
        WHERE start_time IS NOT NULL 
              AND start_time >= FROM_UNIXTIME(?) 
              AND start_time <= FROM_UNIXTIME(?)
    ),
    first_records AS (
        SELECT DISTINCT {partition_by_clause} AS record_date
        FROM earninginterval
        ORDER BY record_date
        LIMIT ?  -- Get only the first N distinct records based on the interval
    ),
    last_price_per_day AS (
        SELECT 
            {partition_by_clause} AS record_date,
            ep.pool,
            e.rune_price_usd,
            ROW_NUMBER() OVER (PARTITION BY {partition_by_clause}, ep.pool ORDER BY e.start_time DESC) AS rn
        FROM 
            earninginterval e
        LEFT JOIN 
            EarningsPools ep ON e.id = ep.earnings_id
    )
    SELECT 
        CAST(fd.record_date AS CHAR) AS record_date,
        ep.pool,
        MAX(lr.rune_price_usd) AS last_rune_price_usd,  -- Get the last rune_price_usd
        MIN(e.start_time) AS partition_start_time,  -- Aggregate start_time for partition start
        MAX(e.end_time) AS partition_end_time,      -- Aggregate end_time for partition end
        SUM(e.block_rewards) AS total_block_rewards,
        SUM(e.bonding_earnings) AS total_bonding_earnings,
        SUM(e.earnings) AS total_earnings,
        SUM(e.liquidity_earnings) AS total_liquidity_earnings,
        SUM(e.liquidity_fees) AS total_liquidity_fees,
        SUM(ep.asset_liquidity_fees) AS total_asset_liquidity_fees,
        SUM(ep.rune_liquidity_fees) AS total_rune_liquidity_fees,
        SUM(ep.total_liquidity_fees_rune) AS total_liquidity_fees_rune,
        SUM(ep.saver_earning) AS total_saver_earning,
        SUM(ep.rewards) AS total_rewards
    FROM 
        earninginterval e
    LEFT JOIN 
        EarningsPools ep ON e.id = ep.earnings_id
    INNER JOIN 
        first_records fd ON {partition_by_clause} = fd.record_date  -- Join with the first N distinct records
    LEFT JOIN 
        last_price_per_day lr ON fd.record_date = lr.record_date AND ep.pool = lr.pool AND lr.rn = 1  -- Get last rune_price_usd for each pool
    GROUP BY 
        fd.record_date, ep.pool  -- Group by the dynamic partition interval and pool
    ORDER BY 
        fd.record_date, ep.pool;  -- Order by the dynamic partition interval and pool
"#
    );

    let records: Vec<EarningsResponse> = match sqlx::query_as::<_, EarningsResponse>(&query_str)
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
        // return Err(sqlx::Error::RowNotFound);
    }
    // Use a HashMap to store intervals with the start_time as the key
    let mut interval_map: HashMap<String, EarningIntervalData> = HashMap::new();
    for record in records {
        let (
            record_date,
            pool,
            last_rune_price_usd,
            partition_start_time,
            partition_end_time,
            total_block_rewards,
            total_bonding_earnings,
            total_earnings,
            total_liquidity_earnings,
            total_liquidity_fees,
            total_asset_liquidity_fees,
            total_rune_liquidity_fees,
            total_liquidity_fees_rune,
            total_saver_earning,
            total_rewards,
        ) = get(&record);

        let key = record_date.clone();
        // Check if the interval already exists
        let entry = interval_map
            .entry(key)
            .or_insert_with(|| EarningIntervalData {
                start_time: partition_start_time.clone().to_string(),
                end_time: partition_end_time.clone(),
                liquidity_fees: total_liquidity_fees.clone(),
                block_rewards: total_block_rewards.clone(),
                earnings: total_earnings.clone(),
                bonding_earnings: total_bonding_earnings.clone(),
                liquidity_earnings: total_liquidity_earnings.clone(),
                avg_node_count: String::new(), // Fill this if available
                rune_price_usd: last_rune_price_usd.clone(),
                pools: Vec::new(),
            });

        // Push the pool data
        entry.pools.push(PoolData {
            pool: pool.clone(),
            asset_liquidity_fees: Decimal::from_str(&total_asset_liquidity_fees).unwrap(),
            rune_liquidity_fees: Decimal::from_str(&total_rune_liquidity_fees).unwrap(),
            total_liquidity_fees_rune: Decimal::from_str(&total_liquidity_fees_rune).unwrap(),
            saver_earning: Decimal::from_str(&total_saver_earning).unwrap(),
            rewards: Decimal::from_str(&total_rewards).unwrap(),
            earnings: Decimal::from_str(&total_earnings).unwrap(),
        });
    }

    // Convert the HashMap to a Vec
    let intervals: Vec<EarningIntervalData> = interval_map.into_iter().map(|(_, v)| v).collect();
    println!("Intervals: {:?}", intervals.len());
    let mut pool_map: HashMap<String, PoolData> = HashMap::new(); // To hold aggregated pool data
    let mut total_liquidity_fees = Decimal::new(0, 0);
    let mut total_block_rewards = Decimal::new(0, 0);
    let mut total_earnings = Decimal::new(0, 0);
    let mut total_bonding_earnings = Decimal::new(0, 0);
    let mut total_liquidity_earnings = Decimal::new(0, 0);
    let mut total_avg_node_count = Decimal::new(0, 0);
    let mut total_rune_price_usd = Decimal::new(0, 0);
    let mut pool_count = 0;

    for interval in &intervals {
        total_liquidity_fees +=
            Decimal::from_str(&interval.liquidity_fees).unwrap_or_else(|_| Decimal::new(0, 0));
        total_block_rewards +=
            Decimal::from_str(&interval.block_rewards).unwrap_or_else(|_| Decimal::new(0, 0));
        total_earnings +=
            Decimal::from_str(&interval.earnings).unwrap_or_else(|_| Decimal::new(0, 0));
        total_bonding_earnings +=
            Decimal::from_str(&interval.bonding_earnings).unwrap_or_else(|_| Decimal::new(0, 0));
        total_liquidity_earnings +=
            Decimal::from_str(&interval.liquidity_earnings).unwrap_or_else(|_| Decimal::new(0, 0));

        // Assuming avg_node_count is stored as a string and you want to calculate the average
        total_avg_node_count +=
            Decimal::from_str(&interval.avg_node_count).unwrap_or_else(|_| Decimal::new(0, 0));

        for pool in &interval.pools {
            // If the pool is already in the map, create a new entry with aggregated values
            let entry = pool_map.entry(pool.pool.clone()).or_insert(PoolData {
                pool: pool.pool.clone(),
                asset_liquidity_fees: Decimal::new(0, 0),
                rune_liquidity_fees: Decimal::new(0, 0),
                total_liquidity_fees_rune: Decimal::new(0, 0),
                saver_earning: Decimal::new(0, 0),
                rewards: Decimal::new(0, 0),
                earnings: Decimal::new(0, 0),
            });

            // Create a new PoolData to hold the aggregated values
            let new_pool_data = PoolData {
                pool: pool.pool.clone(), // Retain the original pool name
                asset_liquidity_fees: entry.asset_liquidity_fees
                    + Decimal::from_str(&pool.asset_liquidity_fees.to_string())
                        .unwrap_or_else(|_| Decimal::new(0, 0)),
                rune_liquidity_fees: entry.rune_liquidity_fees
                    + Decimal::from_str(&pool.rune_liquidity_fees.to_string())
                        .unwrap_or_else(|_| Decimal::new(0, 0)),
                total_liquidity_fees_rune: entry.total_liquidity_fees_rune
                    + Decimal::from_str(&pool.total_liquidity_fees_rune.to_string())
                        .unwrap_or_else(|_| Decimal::new(0, 0)),
                saver_earning: entry.saver_earning
                    + Decimal::from_str(&pool.saver_earning.to_string())
                        .unwrap_or_else(|_| Decimal::new(0, 0)),
                rewards: entry.rewards
                    + Decimal::from_str(&pool.rewards.to_string())
                        .unwrap_or_else(|_| Decimal::new(0, 0)),
                earnings: entry.earnings
                    + Decimal::from_str(&pool.earnings.to_string())
                        .unwrap_or_else(|_| Decimal::new(0, 0)),
            };

            // Update the entry with the new aggregated values
            *entry = new_pool_data;
        }
    }
    // Convert the pool_map values to a vector of PoolData
    let common_pools: Vec<PoolData> = pool_map.values().cloned().collect();
    println!("total_avg_node_count: {:?}", total_avg_node_count);
    // Now, populate the Responsemeta with the aggregated pools
    let response_meta = EarningMetaData {
        start_time: from.to_string(),
        end_time: to.to_string(),
        liquidity_fees: total_liquidity_fees.to_string(),
        block_rewards: total_block_rewards.to_string(),
        earnings: total_earnings.to_string(),
        bonding_earnings: total_bonding_earnings.to_string(),
        liquidity_earnings: total_liquidity_earnings.to_string(),
        avg_node_count: (total_avg_node_count / Decimal::from(intervals.len() as i64)).to_string(),
        rune_price_usd: intervals.last().unwrap().rune_price_usd.clone(),
        pools: common_pools,
    };

    let response_data = ResponseData {
        meta: response_meta,
        intervals: intervals,
    };

    Ok(response_data)
}

fn get(
    record: &EarningsResponse,
) -> (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
) {
    (
        record.record_date.to_string().clone(),
        record.pool.to_string().clone(),
        record.last_rune_price_usd.to_string().clone(),
        record.partition_start_time.to_string().clone(),
        record.partition_end_time.to_string().clone(),
        record.total_block_rewards.to_string().clone(),
        record.total_bonding_earnings.to_string().clone(),
        record.total_earnings.to_string().clone(),
        record.total_liquidity_earnings.to_string().clone(),
        record.total_liquidity_fees.to_string().clone(),
        record.total_asset_liquidity_fees.to_string().clone(),
        record.total_rune_liquidity_fees.to_string().clone(),
        record.total_liquidity_fees_rune.to_string().clone(),
        record.total_saver_earning.to_string().clone(),
        record.total_rewards.to_string().clone(),
    )
}
