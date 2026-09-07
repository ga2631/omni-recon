use axum::{extract::State, Json};
use omni_common::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

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
        Json(json!({
            "success": true,
            "data": {
                "token": "demo-jwt-access-token-omnirecon",
                "user": {
                    "id": "00000000-0000-0000-0000-000000000002",
                    "email": "admin@omnirecon.local",
                    "full_name": "System Administrator",
                    "role": "ADMIN"
                }
            }
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
