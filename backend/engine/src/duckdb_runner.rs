use crate::parsers::StandardSettlementRecord;
use duckdb::Connection;
use omni_common::{OmniError, Result};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::path::Path;
use tracing::info;

pub struct DuckDbRunner {
    conn: Connection,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FinancialSanitySummary {
    pub total_rows: usize,
    pub total_gross: Decimal,
    pub total_net: Decimal,
    pub total_fees: Decimal,
    pub balanced_rows: usize,
    pub discrepant_rows: usize,
}

impl DuckDbRunner {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)
            .map_err(|e| OmniError::Internal(format!("Failed to open DuckDB: {}", e)))?;

        conn.execute_batch(
            "PRAGMA threads=4;
             PRAGMA memory_limit='4GB';
             PRAGMA preserve_insertion_order=false;",
        )
        .map_err(|e| OmniError::Internal(format!("DuckDB pragma error: {}", e)))?;

        info!("DuckDB analytical engine initialized successfully");
        Ok(Self { conn })
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| OmniError::Internal(format!("Failed to open in-memory DuckDB: {}", e)))?;
        Ok(Self { conn })
    }

    pub fn execute_query(&self, sql: &str) -> Result<usize> {
        self.conn
            .execute(sql, [])
            .map_err(|e| OmniError::Reconciliation(format!("DuckDB query failed: {}", e)))
    }

    /// Stage in-memory slice of StandardSettlementRecord into DuckDB for fast OLAP verification
    pub fn stage_settlement_records(&self, records: &[StandardSettlementRecord]) -> Result<()> {
        self.conn.execute_batch(
            "DROP TABLE IF EXISTS staging_settlement_records;
             CREATE TABLE staging_settlement_records (
                 order_id VARCHAR,
                 platform VARCHAR,
                 payout_id VARCHAR,
                 transaction_type VARCHAR,
                 order_status VARCHAR,
                 buyer_username VARCHAR,
                 tracking_number VARCHAR,
                 gross_amount DOUBLE,
                 seller_discount DOUBLE,
                 platform_voucher DOUBLE,
                 buyer_shipping_fee DOUBLE,
                 seller_shipping_fee DOUBLE,
                 shipping_subsidy DOUBLE,
                 commission_fee DOUBLE,
                 service_fee DOUBLE,
                 payment_fee DOUBLE,
                 affiliate_commission_fee DOUBLE,
                 other_fees DOUBLE,
                 net_settlement DOUBLE,
                 ordered_at TIMESTAMP,
                 delivered_at TIMESTAMP,
                 settled_at TIMESTAMP,
                 raw_attributes VARCHAR,
                 raw_fee_breakdown VARCHAR
             );"
        ).map_err(|e| OmniError::Internal(format!("Failed to create DuckDB staging table: {}", e)))?;

        let mut appender = self.conn.appender("staging_settlement_records")
            .map_err(|e| OmniError::Internal(format!("Failed to create DuckDB appender: {}", e)))?;

        for r in records {
            let gross = r.gross_amount.to_f64().unwrap_or(0.0);
            let s_disc = r.seller_discount.to_f64().unwrap_or(0.0);
            let p_vouch = r.platform_voucher.to_f64().unwrap_or(0.0);
            let b_ship = r.buyer_shipping_fee.to_f64().unwrap_or(0.0);
            let s_ship = r.seller_shipping_fee.to_f64().unwrap_or(0.0);
            let ship_sub = r.shipping_subsidy.to_f64().unwrap_or(0.0);
            let comm = r.commission_fee.to_f64().unwrap_or(0.0);
            let serv = r.service_fee.to_f64().unwrap_or(0.0);
            let pay = r.payment_fee.to_f64().unwrap_or(0.0);
            let aff = r.affiliate_commission_fee.to_f64().unwrap_or(0.0);
            let other = r.other_fees.to_f64().unwrap_or(0.0);
            let net = r.net_settlement.to_f64().unwrap_or(0.0);

            let raw_attrs_str = r.raw_attributes.to_string();
            let raw_fee_str = r.raw_fee_breakdown.to_string();

            let ordered_str = r.ordered_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string());
            let delivered_str = r.delivered_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string());
            let settled_str = r.settled_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string());

            appender.append_row(duckdb::params![
                r.order_id,
                r.platform,
                r.payout_id,
                r.transaction_type,
                r.order_status,
                r.buyer_username,
                r.tracking_number,
                gross,
                s_disc,
                p_vouch,
                b_ship,
                s_ship,
                ship_sub,
                comm,
                serv,
                pay,
                aff,
                other,
                net,
                ordered_str,
                delivered_str,
                settled_str,
                raw_attrs_str,
                raw_fee_str
            ]).map_err(|e| OmniError::Internal(format!("Failed to append row to DuckDB: {}", e)))?;
        }

        appender.flush().map_err(|e| OmniError::Internal(format!("Failed to flush DuckDB appender: {}", e)))?;
        info!("Staged {} records into DuckDB memory", records.len());
        Ok(())
    }

    /// Run financial assertion & balance check on staged settlement data
    pub fn verify_financial_integrity(&self) -> Result<FinancialSanitySummary> {
        let sql = "
            SELECT 
                COUNT(*) AS total_rows,
                COALESCE(SUM(gross_amount), 0) AS total_gross,
                COALESCE(SUM(net_settlement), 0) AS total_net,
                COALESCE(SUM(commission_fee + service_fee + payment_fee + affiliate_commission_fee + seller_shipping_fee + other_fees), 0) AS total_fees,
                COUNT(CASE WHEN ABS((gross_amount - seller_discount - commission_fee - service_fee - payment_fee - affiliate_commission_fee - seller_shipping_fee + shipping_subsidy - other_fees) - net_settlement) <= 100.0 THEN 1 END) AS balanced_rows,
                COUNT(CASE WHEN ABS((gross_amount - seller_discount - commission_fee - service_fee - payment_fee - affiliate_commission_fee - seller_shipping_fee + shipping_subsidy - other_fees) - net_settlement) > 100.0 THEN 1 END) AS discrepant_rows
            FROM staging_settlement_records;
        ";

        let mut stmt = self.conn.prepare(sql)
            .map_err(|e| OmniError::Internal(format!("Prepare error: {}", e)))?;

        let summary = stmt.query_row([], |row| {
            let total_rows: i64 = row.get(0)?;
            let total_gross: f64 = row.get(1)?;
            let total_net: f64 = row.get(2)?;
            let total_fees: f64 = row.get(3)?;
            let balanced_rows: i64 = row.get(4)?;
            let discrepant_rows: i64 = row.get(5)?;

            Ok(FinancialSanitySummary {
                total_rows: total_rows as usize,
                total_gross: Decimal::from_f64_retain(total_gross).unwrap_or(Decimal::ZERO),
                total_net: Decimal::from_f64_retain(total_net).unwrap_or(Decimal::ZERO),
                total_fees: Decimal::from_f64_retain(total_fees).unwrap_or(Decimal::ZERO),
                balanced_rows: balanced_rows as usize,
                discrepant_rows: discrepant_rows as usize,
            })
        }).map_err(|e| OmniError::Internal(format!("Query row error: {}", e)))?;

        info!("Financial Sanity Summary: {:?}", summary);
        Ok(summary)
    }

    pub fn export_to_parquet(&self, table_name: &str, parquet_path: &str) -> Result<()> {
        let sql = format!(
            "COPY {} TO '{}' (FORMAT PARQUET, CODEC 'SNAPPY');",
            table_name, parquet_path
        );
        self.execute_query(&sql)?;
        info!("Exported table {} to Parquet at {}", table_name, parquet_path);
        Ok(())
    }

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }
}
