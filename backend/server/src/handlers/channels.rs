use axum::Json;
use serde_json::json;

pub async fn list_channels_handler() -> Json<serde_json::Value> {
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
