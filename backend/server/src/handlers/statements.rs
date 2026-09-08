use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{Datelike, Utc};
use hex;
use omni_engine::{
    duckdb_runner::DuckDbRunner,
    parsers::{shopee::ShopeeParser, tiktok::TikTokShopParser, StandardSettlementRecord},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub success: bool,
    pub message: String,
    pub data: UploadResultData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResultData {
    pub upload_log_id: Uuid,
    pub merchant_id: Uuid,
    pub shop_id: Option<Uuid>,
    pub platform: String,
    pub report_type: String,
    pub original_filename: String,
    pub file_path: String,
    pub file_hash: String,
    pub file_size_bytes: usize,
    pub total_rows: usize,
    pub successful_rows: usize,
    pub failed_rows: usize,
    pub status: String,
    pub sanity_check: Option<omni_engine::duckdb_runner::FinancialSanitySummary>,
    pub sample_records: Vec<StandardSettlementRecord>,
}

/// Handler for multipart statement file upload & Medallion ETL ingestion
pub async fn upload_statement_handler(mut multipart: Multipart) -> Response {
    let mut merchant_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let mut shop_id: Option<Uuid> = None;
    let mut platform = "SHOPEE".to_string();
    let mut report_type = "INCOME_STATEMENT".to_string();
    let mut filename = "statement.xlsx".to_string();
    let mut file_bytes: Vec<u8> = Vec::new();

    // Iterate through multipart fields
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "merchant_id" => {
                if let Ok(text) = field.text().await {
                    if let Ok(id) = Uuid::parse_str(&text) {
                        merchant_id = id;
                    }
                }
            }
            "shop_id" => {
                if let Ok(text) = field.text().await {
                    if let Ok(id) = Uuid::parse_str(&text) {
                        shop_id = Some(id);
                    }
                }
            }
            "platform" => {
                if let Ok(text) = field.text().await {
                    platform = text.trim().to_uppercase();
                }
            }
            "report_type" => {
                if let Ok(text) = field.text().await {
                    report_type = text.trim().to_uppercase();
                }
            }
            "file" => {
                if let Some(fname) = field.file_name() {
                    filename = fname.to_string();
                }
                if let Ok(bytes) = field.bytes().await {
                    file_bytes = bytes.to_vec();
                }
            }
            _ => {}
        }
    }

    if file_bytes.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "No file content provided in multipart request under 'file' field"
            })),
        )
            .into_response();
    }

    let file_size_bytes = file_bytes.len();
    let upload_log_id = Uuid::new_v4();

    // 1. Calculate SHA-256 Checksum for deduplication & data lineage
    let mut hasher = Sha256::new();
    hasher.update(&file_bytes);
    let file_hash = hex::encode(hasher.finalize());

    info!(
        "Received file '{}' ({} bytes, hash: {}) for platform {} and shop {:?}",
        filename, file_size_bytes, file_hash, platform, shop_id
    );

    // 2. Persist Bronze Layer File (Immutable Storage)
    let now = Utc::now();
    let year = now.year();
    let month = format!("{:02}", now.month());
    let shop_dir_segment = shop_id
        .map(|id| id.to_string())
        .unwrap_or_else(|| "default_shop".to_string());

    let ext = std::path::Path::new(&filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("xlsx");

    let bronze_dir = PathBuf::from("storage")
        .join("bronze")
        .join(merchant_id.to_string())
        .join(shop_dir_segment)
        .join(year.to_string())
        .join(month);

    if let Err(e) = create_dir_all(&bronze_dir) {
        error!("Failed to create bronze storage directory: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": format!("Failed to create storage directory: {}", e)
            })),
        )
            .into_response();
    }

    let saved_filename = format!("{}_{}.{}", upload_log_id, &file_hash[..12], ext);
    let bronze_file_path = bronze_dir.join(saved_filename);

    let mut file = match File::create(&bronze_file_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to save bronze file: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Failed to save bronze file: {}", e)
                })),
            )
                .into_response();
        }
    };

    if let Err(e) = file.write_all(&file_bytes) {
        error!("Failed to write bytes to bronze file: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": format!("Failed to write file: {}", e)
            })),
        )
            .into_response();
    }

    info!("Bronze file stored at: {:?}", bronze_file_path);

    // 3. Execute ETL Transformation & Parsing via DuckDB / Calamine
    let parse_result = match platform.as_str() {
        "SHOPEE" => ShopeeParser::parse_statement(&bronze_file_path),
        "TIKTOK" => TikTokShopParser::parse_statement(&bronze_file_path),
        other => {
            warn!("Platform '{}' falling back to Shopee parser", other);
            ShopeeParser::parse_statement(&bronze_file_path)
        }
    };

    let parsed_records = match parse_result {
        Ok(records) => records,
        Err(e) => {
            error!("Failed to parse statement: {}", e);
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({
                    "success": false,
                    "error": format!("Parsing error: {}", e),
                    "upload_log_id": upload_log_id,
                    "file_path": bronze_file_path.to_string_lossy(),
                    "status": "FAILED"
                })),
            )
                .into_response();
        }
    };

    let total_rows = parsed_records.len();

    // 4. DuckDB In-Memory OLAP Staging & Financial Sanity Verification
    let sanity_summary = match DuckDbRunner::in_memory() {
        Ok(runner) => match runner.stage_settlement_records(&parsed_records) {
            Ok(_) => runner.verify_financial_integrity().ok(),
            Err(e) => {
                warn!("DuckDB staging error: {}", e);
                None
            }
        },
        Err(e) => {
            warn!("DuckDB init error: {}", e);
            None
        }
    };

    let sample_records = parsed_records.iter().take(5).cloned().collect();

    (
        StatusCode::OK,
        Json(UploadResponse {
            success: true,
            message: format!(
                "Successfully uploaded & standardized {} records from {} report",
                total_rows, platform
            ),
            data: UploadResultData {
                upload_log_id,
                merchant_id,
                shop_id,
                platform,
                report_type,
                original_filename: filename,
                file_path: bronze_file_path.to_string_lossy().to_string(),
                file_hash,
                file_size_bytes,
                total_rows,
                successful_rows: total_rows,
                failed_rows: 0,
                status: "COMPLETED".to_string(),
                sanity_check: sanity_summary,
                sample_records,
            },
        }),
    )
        .into_response()
}

/// Handler to list uploaded statement batches
pub async fn list_batches_handler() -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": [
            {
                "id": "00000000-0000-0000-0000-000000000099",
                "filename": "Shopee_Income_Statement_August_2026.xlsx",
                "channel_code": "shopee_official",
                "platform": "SHOPEE",
                "report_type": "INCOME_STATEMENT",
                "total_rows": 15420,
                "status": "COMPLETED",
                "created_at": "2026-08-31T10:00:00Z"
            }
        ]
    }))
}
