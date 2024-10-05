
use std::str::EncodeUtf16;

use crate::adaptors::webhandlers::queries::getswapquery;

use chrono::Utc;
use actix_web::{web::{Data,Json,Query}, HttpResponse, Responder,post,get,};
use serde::{Deserialize, Serialize};

use sqlx::{types::Decimal, FromRow};
use sqlx::query;
use sqlx::Error;
use sqlx::MySqlPool;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime};
use sqlx::types::time::PrimitiveDateTime;


#[derive(Deserialize)]
struct SwapsQuery {
    interval: Option<String>,
    count: Option<i64>,
    from: Option<i64>,
    to: Option<i64>,
}
#[derive(Debug)]
 struct SwapsInterval {
     average_slip: Decimal,
     end_time: DateTime<Utc>,
     from_trade_average_slip: Decimal,
     from_trade_count: Decimal,
     from_trade_fees: i64,
     from_trade_volume: i64,
     from_trade_volume_usd: Decimal,
     start_time: DateTime<Utc>,
     rune_price_usd: Decimal,
     synth_mint_average_slip: Decimal,
     synth_mint_count: i64,
     synth_mint_fees: Decimal,
     synth_mint_volume: Decimal,
     synth_mint_volume_usd: Decimal,
     synth_redeem_average_slip: Decimal,
     synth_redeem_count: Decimal,
     synth_redeem_fees: Decimal,
     synth_redeem_volume: Decimal,
     synth_redeem_volume_usd: Decimal,
     to_asset_average_slip: Decimal,
     to_asset_count: i64,
     to_asset_fees: i64,
     to_asset_volume: i64,
     to_asset_volume_usd: i64,
     to_rune_average_slip: Decimal,
     to_rune_count: i64,
     to_rune_fees: i64,
     to_rune_volume: i64,
     to_rune_volume_usd: Decimal,
     to_trade_average_slip: Decimal,
     to_trade_count: i64,
     to_trade_fees: i64,
     to_trade_volume: i64,
     to_trade_volume_usd: Decimal,
     total_count: Decimal,
     total_fees: Decimal,
     total_volume: Decimal,
     total_volume_usd: Decimal,
}

 #[derive(Debug, Serialize, Deserialize)
]
 struct SwapsMeta {
     average_slip: String,
     end_time: String,
     from_trade_average_slip: String,
     from_trade_count: String,
     from_trade_fees: String,
     from_trade_volume: String,
     from_trade_volume_usd: String,
     rune_price_usd: String,
     start_time: String,
     synth_mint_average_slip: String,
     synth_mint_count: String,
     synth_mint_fees: String,
     synth_mint_volume: String,
     synth_mint_volume_usd: String,
     synth_redeem_average_slip: String,
     synth_redeem_count: String,
     synth_redeem_fees: String,
     synth_redeem_volume: String,
     synth_redeem_volume_usd: String,
     to_asset_average_slip: String,
     to_asset_count: String,
     to_asset_fees: String,
     to_asset_volume: String,
     to_asset_volume_usd: String,
     to_rune_average_slip: String,
     to_rune_count: String,
     to_rune_fees: String,
     to_rune_volume: String,
     to_rune_volume_usd: String,
     to_trade_average_slip: String,
     to_trade_count: String,
     to_trade_fees: String,
     to_trade_volume: String,
     to_trade_volume_usd: String,
     total_count: String,
     total_fees: String,
     total_volume: String,
     total_volume_usd: String,
     next: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
struct SwapsIntervalResponse {
     average_slip: String,
     end_time: String,
     from_trade_average_slip: String,
     from_trade_count: String,
     from_trade_fees: String,
     from_trade_volume: String,
     from_trade_volume_usd: String,
     start_time: String,
     rune_price_usd: String,
     synth_mint_average_slip: String,
     synth_mint_count: String,
     synth_mint_fees: String,
     synth_mint_volume: String,
     synth_mint_volume_usd: String,
     synth_redeem_average_slip: String,
     synth_redeem_count: String,
     synth_redeem_fees: String,
     synth_redeem_volume: String,
     synth_redeem_volume_usd: String,
     to_asset_average_slip: String,
     to_asset_count: String,
     to_asset_fees: String,
     to_asset_volume: String,
     to_asset_volume_usd: String,
     to_rune_average_slip: String,
     to_rune_count: String,
     to_rune_fees: String,
     to_rune_volume: String,
     to_rune_volume_usd: String,
     to_trade_average_slip: String,
     to_trade_count: String,
     to_trade_fees: String,
     to_trade_volume: String,
     to_trade_volume_usd: String,
     total_count: String,
     total_fees: String,
     total_volume: String,
     total_volume_usd: String,
}



#[derive(Debug, Serialize, Deserialize)]
struct SwapResponse {
    intervals: Vec<SwapsIntervalResponse>,               // List of interval structs
    meta: SwapsMeta                              // Meta struct for aggregated data
}




#[derive(Debug, FromRow)
]
 struct SwapGroup {
    // Record Date
     record_date: String,
    // First Record Fields
     first_start_time: DateTime<Utc>, // TIMESTAMP
     first_end_time: DateTime<Utc>, // TIMESTAMP
     first_average_slip: Decimal, // DECIMAL(20, 10)
     first_from_trade_average_slip: Decimal, // DECIMAL(20, 10)
     first_from_trade_count: i64, // INT
     first_from_trade_fees: i64, // BIGINT
     first_from_trade_volume: i64, // BIGINT
     first_from_trade_volume_usd: Decimal, // DECIMAL(20, 10)
     first_rune_price_usd: Decimal, // DECIMAL(20, 10)
     first_synth_mint_average_slip: Decimal, // DECIMAL(20, 10)
     first_synth_mint_count: i64, // INT
     first_synth_mint_fees: i64, // BIGINT
     first_synth_mint_volume: i64, // BIGINT
     first_synth_mint_volume_usd: Decimal, // DECIMAL(20, 10)
     first_synth_redeem_average_slip: Decimal, // DECIMAL(20, 10)
     first_synth_redeem_count: i64, // INT
     first_synth_redeem_fees: i64, // BIGINT
     first_synth_redeem_volume: i64, // BIGINT
     first_synth_redeem_volume_usd: Decimal, // DECIMAL(20, 10)
     first_to_asset_average_slip: Decimal, // DECIMAL(20, 10)
     first_to_asset_count: i64, // INT
     first_to_asset_fees: i64, // BIGINT
     first_to_asset_volume: i64, // BIGINT
     first_to_asset_volume_usd: Decimal, // DECIMAL(20, 10)
     first_to_rune_average_slip: Decimal, // DECIMAL(20, 10)
     first_to_rune_count: i64, // INT
     first_to_rune_fees: i64, // BIGINT
     first_to_rune_volume: i64, // BIGINT
     first_to_rune_volume_usd: Decimal, // DECIMAL(20, 10)
     first_to_trade_average_slip: Decimal
     , // DECIMAL(20, 10)
     first_to_trade_count: i64, // INT
     first_to_trade_fees: i64, // BIGINT
     first_to_trade_volume: i64, // BIGINT
     first_to_trade_volume_usd: Decimal, // DECIMAL(20, 10)
     first_total_count: i64, // INT
     first_total_fees: i64, // BIGINT
     first_total_volume: i64, // BIGINT
     first_total_volume_usd: Decimal, // DECIMAL(20, 10)
    // Last Record Fields
     last_start_time: DateTime<Utc>, // TIMESTAMP
     last_end_time: DateTime<Utc>, // TIMESTAMP
     last_average_slip: Decimal, // DECIMAL(20, 10)
     last_from_trade_average_slip:Decimal, // DECIMAL(20, 10)
     last_from_trade_count: i64, // INT
     last_from_trade_fees: i64, // BIGINT
     last_from_trade_volume: i64, // BIGINT
     last_from_trade_volume_usd: Decimal, // DECIMAL(20, 10)
     last_rune_price_usd: Decimal, // DECIMAL(20, 10)
     last_synth_mint_average_slip: Decimal, // DECIMAL(20, 10)
     last_synth_mint_count: i64, // INT
     last_synth_mint_fees: i64, // BIGINT
     last_synth_mint_volume: i64, // BIGINT
     last_synth_mint_volume_usd: Decimal, // DECIMAL(20, 10)
     last_synth_redeem_average_slip: Decimal, // DECIMAL(20, 10)
     last_synth_redeem_count: i64, // INT
     last_synth_redeem_fees: i64, // BIGINT
     last_synth_redeem_volume: i64, // BIGINT
     last_synth_redeem_volume_usd:  Decimal, // DECIMAL(20, 10)
     last_to_asset_average_slip: Decimal, // DECIMAL(20, 10)
     last_to_asset_count: i64, // INT
     last_to_asset_fees: i64, // BIGINT
     last_to_asset_volume: i64, // BIGINT
     last_to_asset_volume_usd: Decimal, // DECIMAL(20, 10)
     last_to_rune_average_slip: Decimal, // DECIMAL(20, 10)
     last_to_rune_count: i64, // INT
     last_to_rune_fees: i64, // BIGINT
     last_to_rune_volume: i64, // BIGINT
     last_to_rune_volume_usd: Decimal, // DECIMAL(20, 10)
     last_to_trade_average_slip: Decimal, // DECIMAL(20, 10)
     last_to_trade_count: i64, // INT
     last_to_trade_fees: i64, // BIGINT
     last_to_trade_volume: i64, // BIGINT
     last_to_trade_volume_usd: Decimal, // DECIMAL(20, 10)
     last_total_count: i64, // INT
     last_total_fees: i64, // BIGINT
     last_total_volume: i64, // BIGINT
     last_total_volume_usd: Decimal, // DECIMAL(20, 10)

     synth_mint_fees_sum:Decimal,
     synth_mint_volume_sum:Decimal,
     synth_redeem_volume_usd_sum:Decimal,
     synth_redeem_average_slip_avg:Decimal,
     synth_redeem_count_sum:Decimal,
     synth_redeem_fees_sum:Decimal,
     synth_redeem_volume_sum:Decimal

}

#[get("/history/swaps")]
async fn get_swaps_history(
    pool: Data<MySqlPool>,
    query: Query<SwapsQuery>,
) -> impl Responder {
    let interval = query.interval.clone().unwrap_or_else(|| "day".to_string());
    let count = query.count.unwrap_or(15);

    if count < 1 || count > 400 {
        return HttpResponse::BadRequest().body("Count must be between 1 and 400");
    }

    let to = query.to.unwrap_or_else(|| chrono::Utc::now().timestamp());  // Default to current time
    let from = query.from.unwrap_or_else(|| {
        // Calculate from based on the interval and count if not provided
        match interval.as_str() {
            "hour" => to - Duration::hours(count).num_seconds(),
            "day" => to - Duration::days(count).num_seconds(),
            "week" => to - Duration::weeks(count).num_seconds(),
            "month" => to - Duration::days(30 * count).num_seconds(),
            "quarter" => to - Duration::days(90 * count).num_seconds(),
            "year" => to - Duration::days(365 * count).num_seconds(),
            a => if a.len() ==0 {
                to - Duration::days(count).num_seconds()
            }else{
                -1
            },  // Handle invalid interval
        }
    });
    if from == -1 {
        return HttpResponse::BadRequest().body("Invalid interval");
    }

    let result = fetch_data_for_intervals(&pool, from, to, &interval, count).await;

    match result {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": format!("Error: {:?}", e) })),
    }
}

async fn fetch_data_for_intervals(
    pool: &MySqlPool,
    from: i64,
    to: i64,
    interval: &str,
    count: i64,
) -> Result<SwapResponse, sqlx::Error>{
        println!("interval: {} count {} from {} to {}", interval,count,from,to);
        let partition_by_clause = match interval {
        "hour" => "DATE_FORMAT(start_time, '%Y-%m-%d %H')",   // Group by year, month, day, and hour
        "day" => "DATE(start_time)",                          // Group by day
        "week" => "YEARWEEK(start_time)",                     // Group by year and week
        "month" => "DATE_FORMAT(start_time, '%Y-%m')",        // Group by year and month
        "year" => "YEAR(start_time)",                         // Group by year
        _ => "DATE(start_time)",                              // Default to day
    };

    let query_str=getswapquery(partition_by_clause);
    let mut intervals: Vec<SwapsInterval>=Vec::<SwapsInterval>::new();

    let records: Vec<SwapGroup> = match sqlx::query_as::<_, SwapGroup>(&query_str)
        .bind(from)
        .bind(to)
        .bind(count)
        .fetch_all(pool)
        .await {
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

    for  record in records.iter(){
        intervals.push(SwapsInterval{
            average_slip:record.last_average_slip,
            end_time:record.last_end_time,
            from_trade_average_slip:record.first_from_trade_average_slip,
            from_trade_count:record.first_from_trade_average_slip,
            from_trade_fees:record.first_from_trade_fees,
            from_trade_volume:record.first_from_trade_volume,
            from_trade_volume_usd:record.first_from_trade_volume_usd,
            start_time:record.first_start_time,
            rune_price_usd:record.last_rune_price_usd,
            synth_mint_average_slip:record.last_synth_mint_average_slip,
            synth_mint_count:record.last_synth_mint_count,
            synth_mint_fees:record.synth_mint_fees_sum,
            synth_mint_volume:record.synth_mint_volume_sum,
            synth_mint_volume_usd:record.synth_redeem_volume_usd_sum,
            synth_redeem_average_slip:record.synth_redeem_average_slip_avg,
            synth_redeem_count:record.synth_redeem_count_sum,
            synth_redeem_fees:record.synth_redeem_fees_sum,
            synth_redeem_volume:record.synth_redeem_volume_sum,
            synth_redeem_volume_usd:record.synth_redeem_volume_usd_sum,
            to_asset_average_slip:record.last_to_asset_average_slip,
            to_asset_count:record.last_to_asset_count,
            to_asset_fees:record.last_to_asset_fees,
            to_asset_volume:record.last_to_asset_volume,
            to_asset_volume_usd:record.last_to_asset_volume,
            to_rune_average_slip:record.last_to_rune_average_slip,
            to_rune_count:record.last_to_rune_count,
            to_rune_fees:record.last_to_rune_fees,
            to_rune_volume:record.last_to_rune_volume,
            to_rune_volume_usd:record.last_rune_price_usd,
            to_trade_average_slip:record.last_to_trade_average_slip,
            to_trade_count:record.last_to_trade_count,
            to_trade_fees:record.last_to_trade_fees,
            to_trade_volume:record.last_to_trade_volume,
            to_trade_volume_usd:record.last_to_trade_volume_usd,
            total_count: (Decimal::new(record.last_to_asset_count,0) + Decimal::new(record.last_to_rune_count,0) + Decimal::new(record.last_synth_mint_count,0) + record.synth_redeem_count_sum),
            total_fees: (Decimal::new(record.last_to_asset_fees,0) +Decimal::new(record.last_to_rune_fees,0) +Decimal::new(record.last_synth_mint_fees,0)+record.synth_redeem_fees_sum),
            total_volume:(Decimal::new(record.last_to_asset_volume,0 )+ Decimal::new(record.last_to_rune_volume,0) + record.synth_mint_volume_sum + Decimal::new(record.last_synth_redeem_volume, 0)),
            total_volume_usd: (record.last_total_volume_usd)
        });
    }
    let MetaData=compute_meta(&intervals);   
    
    let mut  responseinterval:Vec<SwapsIntervalResponse>=Vec::<SwapsIntervalResponse>::new();
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
    let mut synth_redeem_count:Decimal = Decimal::ZERO;
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

    let mut total_count:Decimal = Decimal::ZERO;
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

    let end_time=intervals.last().unwrap().end_time.to_string();    
    // Compute averages
    let len = intervals.len() as i64;
    let SwapsMetadata:SwapsMeta=SwapsMeta {
        average_slip: (total_average_slip / Decimal::new(len,0)).to_string(),
        end_time: end_time.clone(),
        from_trade_average_slip: (from_trade_average_slip / Decimal::new(len,0)).to_string(),
        from_trade_count:from_trade_count.to_string(),
        from_trade_fees:from_trade_fees.to_string(),
        from_trade_volume:from_trade_volume.to_string(),
        from_trade_volume_usd: (from_trade_volume_usd / Decimal::new(len,0)).to_string(),
        rune_price_usd: intervals.last().unwrap().rune_price_usd.to_string(),
        start_time: intervals.first().unwrap().start_time.to_string(),
        synth_mint_average_slip: (synth_mint_average_slip / Decimal::new(len,0)).to_string(),
        synth_mint_count:synth_mint_count.to_string(),
        synth_mint_fees:synth_mint_fees.to_string(),
        synth_mint_volume:synth_mint_volume.to_string(),
        synth_mint_volume_usd:synth_mint_volume_usd.to_string(),
        synth_redeem_average_slip: (synth_redeem_average_slip / Decimal::new(len,0)).to_string(),
        synth_redeem_count:synth_redeem_count.to_string(),
        synth_redeem_fees:synth_redeem_fees.to_string(),
        synth_redeem_volume:synth_redeem_volume.to_string(),
        synth_redeem_volume_usd:synth_redeem_volume_usd.to_string(),
        to_asset_average_slip: ((to_asset_average_slip / Decimal::new(len,0)).to_string()),
        to_asset_count:to_asset_count.to_string(),
        to_asset_fees:to_asset_fees.to_string(),
        to_asset_volume:to_asset_volume.to_string(),
        to_asset_volume_usd:to_asset_volume_usd.to_string(),
        to_rune_average_slip: ((to_rune_average_slip / Decimal::new(len,0)).to_string()),
        to_rune_count:to_rune_count.to_string(),
        to_rune_fees:to_rune_fees.to_string(),
        to_rune_volume:to_rune_volume.to_string(),
        to_rune_volume_usd:to_rune_volume_usd.to_string(),
        to_trade_average_slip: (to_trade_average_slip / Decimal::new(len,0)).to_string(),
        to_trade_count:to_trade_count.to_string(),
        to_trade_fees:to_trade_fees.to_string(),
        to_trade_volume:to_trade_volume.to_string(),
        to_trade_volume_usd:to_trade_volume_usd.to_string(),
        total_count:total_count.to_string(),
        total_fees:total_fees.to_string(),
        total_volume:total_volume.to_string(),
        total_volume_usd:total_volume_usd.to_string(),
        next: Some(end_time.parse::<DateTime<Utc>>().map(|dt| dt.timestamp().to_string()).unwrap_or_else(|_| "Invalid timestamp".to_string())),
    };
    SwapsMetadata
}
