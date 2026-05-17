use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("validation failed: {0}")]
    Validation(String),

    #[error("entity not found")]
    NotFound,

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("internal: {0}")]
    Internal(String),
}
