use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("AI extraction error: {0}")]
    Ai(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("Unauthorized: {0}")]
    Auth(String),

    #[error("Resource not found")]
    NotFound,
}

pub type AppResult<T> = std::result::Result<T, AppError>;

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Validation(format!("JSON error: {}", e))
    }
}

impl From<uuid::Error> for AppError {
    fn from(e: uuid::Error) -> Self {
        AppError::Validation(format!("UUID error: {}", e))
    }
}
