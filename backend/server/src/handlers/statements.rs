use axum::{extract::Multipart, Json};
use serde_json::json;
use tracing::info;

pub async fn upload_statement_handler(mut multipart: Multipart) -> Json<serde_json::Value> {
    let mut files_received = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        info!("Received statement file upload: {}", file_name);
        files_received.push(file_name);
    }

    Json(json!({
        "success": true,
        "message": "Statement files uploaded successfully and queued for processing",
        "data": {
            "batch_id": "00000000-0000-0000-0000-000000000099",
            "files": files_received,
            "status": "QUEUED"
        }
    }))
}

pub async fn list_batches_handler() -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": [
            {
                "id": "00000000-0000-0000-0000-000000000099",
                "filename": "Shopee_Income_Statement_August_2026.xlsx",
                "channel_code": "shopee_official",
                "total_rows": 15420,
                "status": "PROCESSED",
                "created_at": "2026-08-31T10:00:00Z"
            }
        ]
    }))
}
