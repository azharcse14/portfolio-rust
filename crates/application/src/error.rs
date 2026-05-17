use domain::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("invalid input: {0}")]
    Invalid(String),

    #[error("internal error: {0}")]
    Internal(String),
}
