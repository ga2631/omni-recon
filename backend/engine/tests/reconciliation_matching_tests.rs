use omni_engine::duckdb_runner::DuckDbRunner;

#[test]
fn test_reconciliation_multi_way_matching() {
    let runner = DuckDbRunner::in_memory().expect("Failed to initialize DuckDB");

    // 1. Create mock orders table in DuckDB
    runner
        .execute_query(
            "CREATE TABLE mock_orders (
                order_id VARCHAR,
                channel_code VARCHAR,
                gross_amount DOUBLE,
                commission_fee DOUBLE,
                net_settlement DOUBLE
            );
            INSERT INTO mock_orders VALUES
                ('ORD_MATCH_001', 'SHOPEE', 200000.0, 10000.0, 190000.0),
                ('ORD_COD_DIFF_002', 'TIKTOK', 500000.0, 25000.0, 475000.0),
                ('ORD_FEE_SURGE_003', 'SHOPEE', 100000.0, 20000.0, 80000.0);",
        )
        .expect("Failed to setup mock_orders");

    // 2. Create mock carrier shipments table in DuckDB
    runner
        .execute_query(
            "CREATE TABLE mock_carrier (
                order_id VARCHAR,
                tracking_code VARCHAR,
                carrier_code VARCHAR,
                cod_amount DOUBLE,
                delivery_status VARCHAR
            );
            INSERT INTO mock_carrier VALUES
                ('ORD_MATCH_001', 'GHN001', 'GHN', 200000.0, 'DELIVERED'),
                ('ORD_COD_DIFF_002', 'GHTK002', 'GHTK', 480000.0, 'DELIVERED'),
                ('ORD_FEE_SURGE_003', 'GHN003', 'GHN', 100000.0, 'DELIVERED'),
                ('ORD_MISSING_004', 'VTP004', 'VIETTEL_POST', 350000.0, 'DELIVERED');",
        )
        .expect("Failed to setup mock_carrier");

    // 3. Execute reconciliation matching SQL
    runner
        .execute_query(
            "CREATE TABLE discrepancy_results AS
             SELECT 
                COALESCE(o.order_id, c.order_id) AS order_id,
                o.channel_code,
                c.tracking_code,
                o.gross_amount,
                o.net_settlement,
                c.cod_amount AS carrier_cod_amount,
                (COALESCE(o.gross_amount, 0) - COALESCE(o.net_settlement, 0)) AS total_fee,
                CASE 
                    WHEN o.net_settlement IS NULL THEN 'MISSING_SETTLEMENT'
                    WHEN c.cod_amount IS NOT NULL AND o.gross_amount IS NOT NULL AND c.cod_amount != o.gross_amount THEN 'COD_MISMATCH'
                    WHEN o.gross_amount > 0 AND (o.commission_fee / o.gross_amount) > 0.15 THEN 'FEE_MISMATCH'
                    ELSE 'MATCHED'
                END AS recon_status,
                CASE
                    WHEN c.cod_amount IS NOT NULL AND o.gross_amount IS NOT NULL AND c.cod_amount != o.gross_amount 
                        THEN ABS(c.cod_amount - o.gross_amount)
                    WHEN o.net_settlement IS NULL THEN COALESCE(c.cod_amount, 0)
                    ELSE 0.0
                END AS discrepancy_amount
             FROM mock_orders o
             FULL OUTER JOIN mock_carrier c ON o.order_id = c.order_id;",
        )
        .expect("Failed to execute reconciliation");

    // 4. Verify results
    let conn = runner.get_connection();
    let mut stmt = conn
        .prepare("SELECT order_id, recon_status, discrepancy_amount FROM discrepancy_results ORDER BY order_id;")
        .unwrap();

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, f64>(2)?,
            ))
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(rows.len(), 4);

    // ORD_COD_DIFF_002: COD is 480k vs Gross 500k -> COD_MISMATCH (20k diff)
    let r_cod = rows.iter().find(|(id, _, _)| id == "ORD_COD_DIFF_002").unwrap();
    assert_eq!(r_cod.1, "COD_MISMATCH");
    assert_eq!(r_cod.2, 20000.0);

    // ORD_FEE_SURGE_003: Fee is 20k on 100k (20% > 15%) -> FEE_MISMATCH
    let r_fee = rows.iter().find(|(id, _, _)| id == "ORD_FEE_SURGE_003").unwrap();
    assert_eq!(r_fee.1, "FEE_MISMATCH");

    // ORD_MATCH_001: Matched
    let r_match = rows.iter().find(|(id, _, _)| id == "ORD_MATCH_001").unwrap();
    assert_eq!(r_match.1, "MATCHED");
    assert_eq!(r_match.2, 0.0);

    // ORD_MISSING_004: Delivered by carrier but not in settlement -> MISSING_SETTLEMENT
    let r_missing = rows.iter().find(|(id, _, _)| id == "ORD_MISSING_004").unwrap();
    assert_eq!(r_missing.1, "MISSING_SETTLEMENT");
}
