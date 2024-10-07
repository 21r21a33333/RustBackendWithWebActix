
pub fn getswapquery(partition_by_clause: &str) -> String {
    let swap_query_str = format!(
        r#"
                            WITH RankedRecords AS (
                    SELECT 
                        *,
                        {} AS record_date,
                        ROW_NUMBER() OVER (PARTITION BY {} ORDER BY start_time) AS rn_first,
                        ROW_NUMBER() OVER (PARTITION BY {} ORDER BY start_time DESC) AS rn_last,
                        sum(synth_mint_fees) OVER (PARTITION BY {} ORDER BY start_time DESC) AS synth_mint_fees_sum,
                        sum(synth_mint_volume) OVER (PARTITION BY {} ORDER BY start_time DESC) AS synth_mint_volume_sum,
                        sum(synth_redeem_volume_usd) OVER (PARTITION BY {} ORDER BY start_time DESC) AS synth_redeem_volume_usd_sum,
                        avg(synth_redeem_average_slip) OVER (PARTITION BY {} ORDER BY start_time DESC) AS synth_redeem_average_slip_avg,
                        sum(synth_redeem_count) OVER (PARTITION BY {} ORDER BY start_time DESC) AS synth_redeem_count_sum,
                        sum(synth_redeem_fees) OVER (PARTITION BY {} ORDER BY start_time DESC) AS synth_redeem_fees_sum,
                        sum(synth_redeem_volume) OVER (PARTITION BY {} ORDER BY start_time DESC) AS synth_redeem_volume_sum

                    FROM Swaps
                    WHERE start_time IS NOT NULL 
                    AND start_time >= FROM_UNIXTIME(?) 
                    AND start_time <= FROM_UNIXTIME(?)
                )

                SELECT 
                    CAST(record_date AS CHAR) AS record_date,
                    MAX(CASE WHEN rn_last = 1 THEN synth_mint_fees_sum END) AS synth_mint_fees_sum,
                    MAX(CASE WHEN rn_last = 1 THEN synth_mint_volume_sum END) AS synth_mint_volume_sum,
                    MAX(CASE WHEN rn_last = 1 THEN synth_redeem_volume_usd_sum END) AS synth_redeem_volume_usd_sum,
                     MAX(CASE WHEN rn_last = 1 THEN synth_redeem_average_slip_avg END) AS synth_redeem_average_slip_avg,
                     MAX(CASE WHEN rn_last = 1 THEN synth_redeem_count_sum END) AS synth_redeem_count_sum,
                    MAX(CASE WHEN rn_last = 1 THEN synth_redeem_fees_sum END) AS synth_redeem_fees_sum,
                    MAX(CASE WHEN rn_last = 1 THEN synth_redeem_volume_usd_sum END) AS synth_redeem_volume_usd_sum,
                    MAX(CASE WHEN rn_last = 1 THEN synth_redeem_volume_sum END) AS synth_redeem_volume_sum,
                    -- First record fields
                    MAX(CASE WHEN rn_first = 1 THEN start_time END) AS first_start_time,
                    MAX(CASE WHEN rn_first = 1 THEN end_time END) AS first_end_time,
                    MAX(CASE WHEN rn_first = 1 THEN average_slip END) AS first_average_slip,
                    MAX(CASE WHEN rn_first = 1 THEN from_trade_average_slip END) AS first_from_trade_average_slip,
                    MAX(CASE WHEN rn_first = 1 THEN from_trade_count END) AS first_from_trade_count,
                    MAX(CASE WHEN rn_first = 1 THEN from_trade_fees END) AS first_from_trade_fees,
                    MAX(CASE WHEN rn_first = 1 THEN from_trade_volume END) AS first_from_trade_volume,
                    MAX(CASE WHEN rn_first = 1 THEN from_trade_volume_usd END) AS first_from_trade_volume_usd,
                    MAX(CASE WHEN rn_first = 1 THEN rune_price_usd END) AS first_rune_price_usd,
                    MAX(CASE WHEN rn_first = 1 THEN synth_mint_average_slip END) AS first_synth_mint_average_slip,
                    MAX(CASE WHEN rn_first = 1 THEN synth_mint_count END) AS first_synth_mint_count,
                    MAX(CASE WHEN rn_first = 1 THEN synth_mint_fees END) AS first_synth_mint_fees,
                    MAX(CASE WHEN rn_first = 1 THEN synth_mint_volume END) AS first_synth_mint_volume,
                    MAX(CASE WHEN rn_first = 1 THEN synth_mint_volume_usd END) AS first_synth_mint_volume_usd,
                    MAX(CASE WHEN rn_first = 1 THEN synth_redeem_average_slip END) AS first_synth_redeem_average_slip,
                    MAX(CASE WHEN rn_first = 1 THEN synth_redeem_count END) AS first_synth_redeem_count,
                    MAX(CASE WHEN rn_first = 1 THEN synth_redeem_fees END) AS first_synth_redeem_fees,
                    MAX(CASE WHEN rn_first = 1 THEN synth_redeem_volume END) AS first_synth_redeem_volume,
                    MAX(CASE WHEN rn_first = 1 THEN synth_redeem_volume_usd END) AS first_synth_redeem_volume_usd,
                    MAX(CASE WHEN rn_first = 1 THEN to_asset_average_slip END) AS first_to_asset_average_slip,
                    MAX(CASE WHEN rn_first = 1 THEN to_asset_count END) AS first_to_asset_count,
                    MAX(CASE WHEN rn_first = 1 THEN to_asset_fees END) AS first_to_asset_fees,
                    MAX(CASE WHEN rn_first = 1 THEN to_asset_volume END) AS first_to_asset_volume,
                    MAX(CASE WHEN rn_first = 1 THEN to_asset_volume_usd END) AS first_to_asset_volume_usd,
                    MAX(CASE WHEN rn_first = 1 THEN to_rune_average_slip END) AS first_to_rune_average_slip,
                    MAX(CASE WHEN rn_first = 1 THEN to_rune_count END) AS first_to_rune_count,
                    MAX(CASE WHEN rn_first = 1 THEN to_rune_fees END) AS first_to_rune_fees,
                    MAX(CASE WHEN rn_first = 1 THEN to_rune_volume END) AS first_to_rune_volume,
                    MAX(CASE WHEN rn_first = 1 THEN to_rune_volume_usd END) AS first_to_rune_volume_usd,
                    MAX(CASE WHEN rn_first = 1 THEN to_trade_average_slip END) AS first_to_trade_average_slip,
                    MAX(CASE WHEN rn_first = 1 THEN to_trade_count END) AS first_to_trade_count,
                    MAX(CASE WHEN rn_first = 1 THEN to_trade_fees END) AS first_to_trade_fees,
                    MAX(CASE WHEN rn_first = 1 THEN to_trade_volume END) AS first_to_trade_volume,
                    MAX(CASE WHEN rn_first = 1 THEN to_trade_volume_usd END) AS first_to_trade_volume_usd,
                    MAX(CASE WHEN rn_first = 1 THEN total_count END) AS first_total_count,
                    MAX(CASE WHEN rn_first = 1 THEN total_fees END) AS first_total_fees,
                    MAX(CASE WHEN rn_first = 1 THEN total_volume END) AS first_total_volume,
                    MAX(CASE WHEN rn_first = 1 THEN total_volume_usd END) AS first_total_volume_usd,

                    -- Last record fields
                    MAX(CASE WHEN rn_last = 1 THEN start_time END) AS last_start_time,
                    MAX(CASE WHEN rn_last = 1 THEN end_time END) AS last_end_time,
                    MAX(CASE WHEN rn_last = 1 THEN average_slip END) AS last_average_slip,
                    MAX(CASE WHEN rn_last = 1 THEN from_trade_average_slip END) AS last_from_trade_average_slip,
                    MAX(CASE WHEN rn_last = 1 THEN from_trade_count END) AS last_from_trade_count,
                    MAX(CASE WHEN rn_last = 1 THEN from_trade_fees END) AS last_from_trade_fees,
                    MAX(CASE WHEN rn_last = 1 THEN from_trade_volume END) AS last_from_trade_volume,
                    MAX(CASE WHEN rn_last = 1 THEN from_trade_volume_usd END) AS last_from_trade_volume_usd,
                    MAX(CASE WHEN rn_last = 1 THEN rune_price_usd END) AS last_rune_price_usd,
                    MAX(CASE WHEN rn_last = 1 THEN synth_mint_average_slip END) AS last_synth_mint_average_slip,
                    MAX(CASE WHEN rn_last = 1 THEN synth_mint_count END) AS last_synth_mint_count,
                    MAX(CASE WHEN rn_last = 1 THEN synth_mint_fees END) AS last_synth_mint_fees,
                    MAX(CASE WHEN rn_last = 1 THEN synth_mint_volume END) AS last_synth_mint_volume,
                    MAX(CASE WHEN rn_last = 1 THEN synth_mint_volume_usd END) AS last_synth_mint_volume_usd,
                    MAX(CASE WHEN rn_last = 1 THEN synth_redeem_average_slip END) AS last_synth_redeem_average_slip,
                    MAX(CASE WHEN rn_last = 1 THEN synth_redeem_count END) AS last_synth_redeem_count,
                    MAX(CASE WHEN rn_last = 1 THEN synth_redeem_fees END) AS last_synth_redeem_fees,
                    MAX(CASE WHEN rn_last = 1 THEN synth_redeem_volume END) AS last_synth_redeem_volume,
                    MAX(CASE WHEN rn_last = 1 THEN synth_redeem_volume_usd END) AS last_synth_redeem_volume_usd,
                    MAX(CASE WHEN rn_last = 1 THEN to_asset_average_slip END) AS last_to_asset_average_slip,
                    MAX(CASE WHEN rn_last = 1 THEN to_asset_count END) AS last_to_asset_count,
                    MAX(CASE WHEN rn_last = 1 THEN to_asset_fees END) AS last_to_asset_fees,
                    MAX(CASE WHEN rn_last = 1 THEN to_asset_volume END) AS last_to_asset_volume,
                    MAX(CASE WHEN rn_last = 1 THEN to_asset_volume_usd END) AS last_to_asset_volume_usd,
                    MAX(CASE WHEN rn_last = 1 THEN to_rune_average_slip END) AS last_to_rune_average_slip,
                    MAX(CASE WHEN rn_last = 1 THEN to_rune_count END) AS last_to_rune_count,
                    MAX(CASE WHEN rn_last = 1 THEN to_rune_fees END) AS last_to_rune_fees,
                    MAX(CASE WHEN rn_last = 1 THEN to_rune_volume END) AS last_to_rune_volume,
                    MAX(CASE WHEN rn_last = 1 THEN to_rune_volume_usd END) AS last_to_rune_volume_usd,
                    MAX(CASE WHEN rn_last = 1 THEN to_trade_average_slip END) AS last_to_trade_average_slip,
                    MAX(CASE WHEN rn_last = 1 THEN to_trade_count END) AS last_to_trade_count,
                    MAX(CASE WHEN rn_last = 1 THEN to_trade_fees END) AS last_to_trade_fees,
                    MAX(CASE WHEN rn_last = 1 THEN to_trade_volume END) AS last_to_trade_volume,
                    MAX(CASE WHEN rn_last = 1 THEN to_trade_volume_usd END) AS last_to_trade_volume_usd,
                    MAX(CASE WHEN rn_last = 1 THEN total_count END) AS last_total_count,
                    MAX(CASE WHEN rn_last = 1 THEN total_fees END) AS last_total_fees,
                    MAX(CASE WHEN rn_last = 1 THEN total_volume END) AS last_total_volume,
                    MAX(CASE WHEN rn_last = 1 THEN total_volume_usd END) AS last_total_volume_usd
                    
                FROM RankedRecords
                GROUP BY record_date
                ORDER BY record_date
                LIMIT ?;

    "#,
        partition_by_clause,
        partition_by_clause,
        partition_by_clause,
        partition_by_clause,
        partition_by_clause,
        partition_by_clause,
        partition_by_clause,
        partition_by_clause,
        partition_by_clause,
        partition_by_clause
    );

    return swap_query_str;
}
