use crate::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChannelItemDto {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub platform_type: String,
    pub is_active: bool,
}

pub async fn list_channels_handler(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    if let Some(ref pool) = state.pool {
        let rows = sqlx::query_as::<_, ChannelItemDto>(
            "SELECT id, code, name, platform as platform_type, is_active FROM shops WHERE is_active = true ORDER BY name ASC"
        )
        .fetch_all(pool)
        .await;

        if let Ok(channels) = rows {
            if !channels.is_empty() {
                return Json(json!({
                    "success": true,
                    "data": channels
                }));
            }
        }
    }

    // Fallback seed channels if DB is empty or unpopulated
    Json(json!({
        "success": true,
        "data": [
            {
                "id": "00000000-0000-0000-0000-000000000010",
                "code": "shopee_official",
                "name": "Gian Hàng Shopee Mall",
                "platform_type": "SHOPEE",
                "is_active": true
            },
            {
                "id": "00000000-0000-0000-0000-000000000011",
                "code": "tiktok_shop_main",
                "name": "TikTok Shop Flagship",
                "platform_type": "TIKTOK",
                "is_active": true
            },
            {
                "id": "00000000-0000-0000-0000-000000000012",
                "code": "ghn_express",
                "name": "Giao Hàng Nhanh (GHN)",
                "platform_type": "GHN",
                "is_active": true
            }
        ]
    }))
}

