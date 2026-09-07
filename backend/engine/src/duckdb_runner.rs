use duckdb::{Connection, Result as DuckResult};
use omni_common::{OmniError, Result};
use std::path::Path;
use tracing::info;

pub struct DuckDbRunner {
    conn: Connection,
}

impl DuckDbRunner {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)
            .map_err(|e| OmniError::Internal(format!("Failed to open DuckDB: {}", e)))?;
        
        // Optimize DuckDB settings for analytical workloads
        conn.execute_batch(
            "PRAGMA threads=4;
             PRAGMA memory_limit='4GB';
             PRAGMA preserve_insertion_order=false;"
        ).map_err(|e| OmniError::Internal(format!("DuckDB pragma error: {}", e)))?;

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
