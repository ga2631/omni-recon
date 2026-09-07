use thiserror::Error;

#[derive(Error, Debug)]
pub enum OmniError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Parser error: {0}")]
    Parser(String),

    #[error("Reconciliation error: {0}")]
    Reconciliation(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, OmniError>;
