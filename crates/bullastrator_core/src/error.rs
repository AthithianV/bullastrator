use thiserror::Error;

pub type Result<T> = std::result::Result<T, BullastratorError>;

#[derive(Debug, Error)]
pub enum BullastratorError {
    #[error("validation error: {0}")]
    Validation(String),

    #[error("resource not found: {0}")]
    NotFound(String),

    #[error("authentication failed")]
    Authentication,

    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl From<&str> for BullastratorError {
    fn from(message: &str) -> Self {
        Self::Validation(message.to_owned())
    }
}

impl From<String> for BullastratorError {
    fn from(message: String) -> Self {
        Self::Validation(message)
    }
}
