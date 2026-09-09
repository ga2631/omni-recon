use crate::duckdb_runner::DuckDbRunner;
use omni_common::Result;
use tracing::info;

pub struct ReconciliationMatcher<'a> {
    runner: &'a DuckDbRunner,
}

impl<'a> ReconciliationMatcher<'a> {
    pub fn new(runner: &'a DuckDbRunner) -> Self {
        Self { runner }
    }

    /// Perform multi-way matching between orders, settlements, and carrier shipments using DuckDB SQL
    pub fn execute_reconciliation(
        &self,
        orders_parquet: &str,
        carrier_parquet: &str,
        output_discrepancies_table: &str,
    ) -> Result<usize> {
        info!("Executing multi-way reconciliation using DuckDB engine...");

        let sql = format!(
            "CREATE OR REPLACE TABLE {} AS
             SELECT 
                COALESCE(o.order_id, c.order_id) AS order_id,
                o.channel_code,
                c.tracking_code,
                o.gross_amount,
                o.net_settlement,
                c.cod_amount AS carrier_cod_amount,
                (o.gross_amount - o.net_settlement) AS total_fee,
                CASE 
                    WHEN o.net_settlement IS NULL THEN 'MISSING_SETTLEMENT'
                    WHEN c.cod_amount IS NOT NULL AND c.cod_amount != o.gross_amount THEN 'COD_MISMATCH'
                    WHEN o.gross_amount > 0 AND (o.commission_fee / o.gross_amount) > 0.15 THEN 'FEE_MISMATCH'
                    ELSE 'MATCHED'
                END AS recon_status,
                CASE
                    WHEN c.cod_amount IS NOT NULL AND c.cod_amount != o.gross_amount 
                        THEN ABS(c.cod_amount - o.gross_amount)
                    ELSE 0.0
                END AS discrepancy_amount
             FROM read_parquet('{}') o
             FULL OUTER JOIN read_parquet('{}') c 
                ON o.order_id = c.order_id
             WHERE recon_status != 'MATCHED';",
            output_discrepancies_table, orders_parquet, carrier_parquet
        );

        let rows_affected = self.runner.execute_query(&sql)?;
        info!("Reconciliation completed. Discrepant rows identified: {}", rows_affected);
        Ok(rows_affected)
    }
}
