use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub app_env: String,
    pub app_port: u16,
    pub app_secret: String,
    pub database_url: String,
    pub redis_url: String,
    pub duckdb_path: String,
    pub storage_path: String,
}

impl AppConfig {
    pub fn load_from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            app_env: env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
            app_port: env::var("APP_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            app_secret: env::var("APP_SECRET")
                .unwrap_or_else(|_| "omni-recon-default-secret-key-32chars".to_string()),
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgresql://omnirecon:omnirecon_secure_password@localhost:5432/omnirecon".to_string()
            }),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379/0".to_string()),
            duckdb_path: env::var("DUCKDB_PATH")
                .unwrap_or_else(|_| "./data/omnirecon.duckdb".to_string()),
            storage_path: env::var("STORAGE_PATH")
                .unwrap_or_else(|_| "./data/storage".to_string()),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::load_from_env()
    }
}

