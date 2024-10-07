use chrono::{DateTime, Utc};
use sqlx::MySqlPool;

use crate::domain::{RUNEPoolHistoryInterval, RUNEPoolHistoryIntervalGroup, RUNEPoolHistoryMeta, RUNEPoolHistoryResponse};

pub async fn fetch_data_for_intervals(
    pool: &MySqlPool,
    from: i64,
    to: i64,
    interval: &str,
    count: i64,
) -> Result<RUNEPoolHistoryResponse, sqlx::Error> {
    // Create the correct SQL PARTITION BY clause based on the interval
    let partition_by_clause = match interval {
        "hour" => "DATE_FORMAT(start_time, '%Y-%m-%d %H')", // Group by year, month, day, and hour
        "day" => "DATE(start_time)",                        // Group by day
        "week" => "YEARWEEK(start_time)",                   // Group by year and week
        "month" => "DATE_FORMAT(start_time, '%Y-%m')",      // Group by year and month
        "year" => "YEAR(start_time)",                       // Group by year
        _ => "DATE(start_time)",                            // Default to day
    };

    // SQL query with the dynamic partition_by_clause
    let query_str = format!(
        r#"
    WITH RankedRecords AS (
        SELECT 
            start_time,
            units,
            count,
            end_time,
            {} AS record_date,  -- Use the dynamic partition clause here
            ROW_NUMBER() OVER (PARTITION BY {} ORDER BY start_time) AS rn_first,
            ROW_NUMBER() OVER (PARTITION BY {} ORDER BY start_time DESC) AS rn_last
        FROM runepool
        WHERE start_time IS NOT NULL 
          AND start_time >= FROM_UNIXTIME(?) 
          AND start_time <= FROM_UNIXTIME(?)
    )
    SELECT 
        CAST(record_date AS CHAR) AS record_date,
        MAX(CASE WHEN rn_first = 1 THEN start_time END) AS first_record,
        MAX(CASE WHEN rn_first = 1 THEN units END) AS first_units,
        MAX(CASE WHEN rn_first = 1 THEN count END) AS first_count,
        MAX(CASE WHEN rn_last = 1 THEN end_time END) AS last_record,
        MAX(CASE WHEN rn_last = 1 THEN units END) AS last_units,
        MAX(CASE WHEN rn_last = 1 THEN count END) AS last_count
    FROM RankedRecords
    GROUP BY record_date
    ORDER BY record_date
    LIMIT ?;
    "#,
        partition_by_clause, partition_by_clause, partition_by_clause
    );

    let mut intervals = Vec::<RUNEPoolHistoryInterval>::new();

    // Execute the query using `sqlx`
    let records: Vec<RUNEPoolHistoryIntervalGroup> =
        match sqlx::query_as::<_, RUNEPoolHistoryIntervalGroup>(&query_str)
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
        return Err(sqlx::Error::RowNotFound); // Return RowNotFound if no records
    }

    // Populate the response intervals
    for record in records.iter() {
        intervals.push(RUNEPoolHistoryInterval {
            start_time: record.first_record.to_string(),
            end_time: record.last_record.to_string(),
            count: record.last_count.to_string(),
            units: record.last_units.to_string(),
        });
    }

    let endtime = records[records.len() - 1].last_record.to_string();
    // Build the metadata
    let metadata = RUNEPoolHistoryMeta {
        start_time: records[0].first_record.to_string(),
        end_time: endtime.clone(), // Clone to avoid borrowing issues
        start_units: records[0].first_units.to_string(),
        start_count: records[0].first_count.to_string(),
        end_units: records[records.len() - 1].last_units.to_string(),
        end_count: records[records.len() - 1].last_count.to_string(),
        next_page: Some(
            endtime
                .parse::<DateTime<Utc>>()
                .map(|dt| dt.timestamp().to_string())
                .unwrap_or_else(|_| "Invalid timestamp".to_string()),
        ),
    };

    // Build the final response
    let returndata = RUNEPoolHistoryResponse {
        intervals,
        meta: metadata,
    };

    Ok(returndata)
}
