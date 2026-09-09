use crate::AppState;
use axum::{
    extract::{Multipart, State},
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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BatchListItem {
    pub id: Uuid,
    pub filename: String,
    pub channel_code: Option<String>,
    pub platform: String,
    pub report_type: String,
    pub total_rows: i32,
    pub status: String,
    pub created_at: chrono::DateTime<Utc>,
}

fn normalize_order_status(raw: &str) -> &'static str {
    let lower = raw.trim().to_lowercase();
    if lower.contains("trả hàng")
        || lower.contains("hoàn tiền")
        || lower.contains("returned")
        || lower.contains("refund")
    {
        "RETURNED"
    } else if lower.contains("hủy")
        || lower.contains("cancelled")
        || lower.contains("canceled")
    {
        "CANCELLED"
    } else if lower.contains("đang giao")
        || lower.contains("giao hàng")
        || lower.contains("delivered")
        || lower.contains("shipping")
    {
        "DELIVERED"
    } else if lower.contains("xử lý")
        || lower.contains("chờ")
        || lower.contains("pending")
        || lower.contains("processing")
    {
        "PROCESSING"
    } else {
        "COMPLETED"
    }
}

/// Handler for multipart statement file upload & Medallion ETL ingestion
pub async fn upload_statement_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Response {
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

    // 5. Persist Silver conformed schema into PostgreSQL (upload_logs, unified_orders, unified_transactions)
    if let Some(ref pool) = state.pool {
        let resolved_shop_id = match shop_id {
            Some(sid) => Some(sid),
            None => {
                let row: Option<(Uuid,)> = sqlx::query_as(
                    "SELECT id FROM shops WHERE merchant_id = $1 AND platform = $2 LIMIT 1",
                )
                .bind(merchant_id)
                .bind(&platform)
                .fetch_optional(pool)
                .await
                .unwrap_or(None);
                row.map(|r| r.0)
            }
        };

        let total_i32 = total_rows as i32;
        let file_size_i64 = file_size_bytes as i64;
        let now_utc = Utc::now();

        // 5.1 Insert/Upsert into upload_logs
        let log_res = sqlx::query(
            r#"
            INSERT INTO upload_logs (
                id, merchant_id, shop_id, platform, report_type,
                original_filename, file_path, file_hash, file_size_bytes,
                total_rows, successful_rows, failed_rows, status,
                error_summary, created_at, processed_at
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7, $8, $9,
                $10, $11, $12, $13,
                $14, $15, $16
            )
            ON CONFLICT (shop_id, file_hash) DO UPDATE SET
                total_rows = EXCLUDED.total_rows,
                successful_rows = EXCLUDED.successful_rows,
                failed_rows = EXCLUDED.failed_rows,
                status = EXCLUDED.status,
                processed_at = EXCLUDED.processed_at
            "#
        )
        .bind(upload_log_id)
        .bind(merchant_id)
        .bind(resolved_shop_id)
        .bind(&platform)
        .bind(&report_type)
        .bind(&filename)
        .bind(bronze_file_path.to_string_lossy().to_string())
        .bind(&file_hash)
        .bind(file_size_i64)
        .bind(total_i32)
        .bind(total_i32)
        .bind(0i32)
        .bind("COMPLETED")
        .bind(serde_json::json!({}))
        .bind(now_utc)
        .bind(Some(now_utc))
        .execute(pool)
        .await;

        if let Err(e) = log_res {
            error!("Failed to persist upload_log: {}", e);
        }

        // 5.2 Insert/Upsert into unified_orders and unified_transactions
        if let Some(effective_shop_id) = resolved_shop_id {
            for rec in &parsed_records {
                let ordered_at_utc = rec.ordered_at.map(|dt| dt.and_utc());
                let delivered_at_utc = rec.delivered_at.map(|dt| dt.and_utc());
                let settled_at_utc = rec.settled_at.map(|dt| dt.and_utc());

                let normalized_status = normalize_order_status(&rec.order_status);

                let order_res = sqlx::query_as::<_, (Uuid,)>(
                    r#"
                    INSERT INTO unified_orders (
                        id, merchant_id, shop_id, upload_log_id, platform,
                        platform_order_id, order_status, buyer_username, tracking_number,
                        ordered_at, delivered_at, raw_attributes, created_at, updated_at
                    ) VALUES (
                        uuid_generate_v4(), $1, $2, $3, $4,
                        $5, $6, $7, $8,
                        $9, $10, $11, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
                    )
                    ON CONFLICT (shop_id, platform_order_id) DO UPDATE SET
                        upload_log_id = EXCLUDED.upload_log_id,
                        order_status = EXCLUDED.order_status,
                        buyer_username = COALESCE(EXCLUDED.buyer_username, unified_orders.buyer_username),
                        tracking_number = COALESCE(EXCLUDED.tracking_number, unified_orders.tracking_number),
                        ordered_at = COALESCE(EXCLUDED.ordered_at, unified_orders.ordered_at),
                        delivered_at = COALESCE(EXCLUDED.delivered_at, unified_orders.delivered_at),
                        raw_attributes = EXCLUDED.raw_attributes,
                        updated_at = CURRENT_TIMESTAMP
                    RETURNING id
                    "#
                )
                .bind(merchant_id)
                .bind(effective_shop_id)
                .bind(Some(upload_log_id))
                .bind(&platform)
                .bind(&rec.order_id)
                .bind(normalized_status)
                .bind(&rec.buyer_username)
                .bind(&rec.tracking_number)
                .bind(ordered_at_utc)
                .bind(delivered_at_utc)
                .bind(&rec.raw_attributes)
                .fetch_optional(pool)
                .await;

                let order_uuid = match order_res {
                    Ok(Some(row)) => Some(row.0),
                    Ok(None) => None,
                    Err(e) => {
                        error!("Failed to persist unified_order for {}: {}", rec.order_id, e);
                        None
                    }
                };

                let payout_str = rec.payout_id.clone().unwrap_or_else(|| "DEFAULT".to_string());

                let tx_res = sqlx::query(
                    r#"
                    INSERT INTO unified_transactions (
                        id, order_id, merchant_id, shop_id, upload_log_id, platform,
                        platform_order_id, payout_id, transaction_type,
                        gross_amount, seller_discount, platform_voucher,
                        buyer_shipping_fee, seller_shipping_fee, shipping_subsidy,
                        commission_fee, service_fee, payment_fee,
                        affiliate_commission_fee, other_fees, net_settlement,
                        settled_at, raw_fee_breakdown, created_at
                    ) VALUES (
                        uuid_generate_v4(), $1, $2, $3, $4, $5,
                        $6, $7, $8,
                        $9, $10, $11,
                        $12, $13, $14,
                        $15, $16, $17,
                        $18, $19, $20,
                        $21, $22, CURRENT_TIMESTAMP
                    )
                    ON CONFLICT (shop_id, platform_order_id, payout_id, transaction_type) DO UPDATE SET
                        order_id = EXCLUDED.order_id,
                        upload_log_id = EXCLUDED.upload_log_id,
                        gross_amount = EXCLUDED.gross_amount,
                        seller_discount = EXCLUDED.seller_discount,
                        platform_voucher = EXCLUDED.platform_voucher,
                        buyer_shipping_fee = EXCLUDED.buyer_shipping_fee,
                        seller_shipping_fee = EXCLUDED.seller_shipping_fee,
                        shipping_subsidy = EXCLUDED.shipping_subsidy,
                        commission_fee = EXCLUDED.commission_fee,
                        service_fee = EXCLUDED.service_fee,
                        payment_fee = EXCLUDED.payment_fee,
                        affiliate_commission_fee = EXCLUDED.affiliate_commission_fee,
                        other_fees = EXCLUDED.other_fees,
                        net_settlement = EXCLUDED.net_settlement,
                        settled_at = EXCLUDED.settled_at,
                        raw_fee_breakdown = EXCLUDED.raw_fee_breakdown
                    "#
                )
                .bind(order_uuid)
                .bind(merchant_id)
                .bind(effective_shop_id)
                .bind(upload_log_id)
                .bind(&platform)
                .bind(&rec.order_id)
                .bind(&payout_str)
                .bind(&rec.transaction_type)
                .bind(rec.gross_amount)
                .bind(rec.seller_discount)
                .bind(rec.platform_voucher)
                .bind(rec.buyer_shipping_fee)
                .bind(rec.seller_shipping_fee)
                .bind(rec.shipping_subsidy)
                .bind(rec.commission_fee)
                .bind(rec.service_fee)
                .bind(rec.payment_fee)
                .bind(rec.affiliate_commission_fee)
                .bind(rec.other_fees)
                .bind(rec.net_settlement)
                .bind(settled_at_utc)
                .bind(&rec.raw_fee_breakdown)
                .execute(pool)
                .await;

                if let Err(e) = tx_res {
                    error!("Failed to persist unified_transaction for order {}: {}", rec.order_id, e);
                }
            }
        }
    }


    let sample_records = parsed_records.iter().take(100).cloned().collect();

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

/// Handler to download sample settlement CSV template conforming to platform standards
pub async fn download_sample_template_handler() -> Response {
    let template_csv = "\u{FEFF}\
Mã đơn hàng,Ngày hoàn thành,Trạng thái đơn hàng,Tổng tiền hàng,Phí vận chuyển người mua trả,Trợ giá phí vận chuyển của Shopee,Phí vận chuyển thực tế,Phí thanh toán,Phí cố định,Phí Dịch Vụ,Số tiền chuyển cho Người bán
260908NORMAL01,2026-09-05 14:00,Hoàn thành,200000,15000,0,15000,8000,8000,10000,174000
260908SHIPPING,2026-09-06 10:30,Hoàn thành,300000,20000,0,45000,12000,12000,15000,236000
260908RETURN03,2026-09-07 09:15,Trả hàng hoàn tiền,150000,25000,15000,40000,0,0,0,0
260908HIDDEN04,2026-09-08 16:45,Hoàn thành,100000,15000,0,15000,4000,4000,5000,70000
";

    (
        StatusCode::OK,
        [
            (axum::http::header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (
                axum::http::header::CONTENT_DISPOSITION,
                "attachment; filename=\"mau_bang_ke_shopee.csv\"",
            ),
        ],
        template_csv,
    )
        .into_response()
}

/// Handler to list uploaded statement batches
pub async fn list_batches_handler(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    if let Some(ref pool) = state.pool {
        let rows = sqlx::query_as::<_, BatchListItem>(
            r#"
            SELECT 
                l.id,
                l.original_filename as filename,
                COALESCE(s.code, LOWER(l.platform)) as channel_code,
                l.platform,
                l.report_type,
                l.total_rows,
                l.status,
                l.created_at
            FROM upload_logs l
            LEFT JOIN shops s ON l.shop_id = s.id
            ORDER BY l.created_at DESC
            LIMIT 50
            "#
        )
        .fetch_all(pool)
        .await;

        if let Ok(batches) = rows {
            return Json(json!({
                "success": true,
                "data": batches
            }));
        }
    }

    Json(json!({
        "success": true,
        "data": []
    }))
}


