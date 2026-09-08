use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub role: String,
}

pub async fn login_handler(
    Json(payload): Json<LoginRequest>,
) -> Json<serde_json::Value> {
    // In production, verify bcrypt password hash with database
    if payload.email == "admin@omnirecon.local" && payload.password == "Admin@123456" {
        let resp = LoginResponse {
            token: "demo-jwt-access-token-omnirecon".to_string(),
            user: UserInfo {
                id: "00000000-0000-0000-0000-000000000002".to_string(),
                email: "admin@omnirecon.local".to_string(),
                full_name: "System Administrator".to_string(),
                role: "ADMIN".to_string(),
            },
        };
        Json(json!({
            "success": true,
            "data": resp
        }))
    } else {
        Json(json!({
            "success": false,
            "message": "Invalid email or password"
        }))
    }
}

pub async fn me_handler() -> Json<serde_json::Value> {
    Json(json!({
        "success": true,
        "data": {
            "id": "00000000-0000-0000-0000-000000000002",
            "email": "admin@omnirecon.local",
            "full_name": "System Administrator",
            "role": "ADMIN",
            "org_name": "OmniRecon Demo Store"
        }
    }))
}
