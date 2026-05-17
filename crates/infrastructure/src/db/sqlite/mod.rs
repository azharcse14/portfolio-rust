//! SQLite-backed repository adapters that implement `domain::ports::*`.

pub mod message_repo;
pub mod post_repo;
pub mod profile_repo;
pub mod project_repo;
pub mod skill_repo;
pub mod user_repo;

pub use message_repo::SqliteMessageRepository;
pub use post_repo::SqlitePostRepository;
pub use profile_repo::SqliteProfileRepository;
pub use project_repo::SqliteProjectRepository;
pub use skill_repo::SqliteSkillRepository;
pub use user_repo::SqliteUserRepository;

use domain::DomainError;

/// Map any sqlx error into a `DomainError`. RowNotFound becomes `NotFound`,
/// everything else becomes `Internal` with the error message preserved.
pub(crate) fn map_sqlx(err: sqlx::Error) -> DomainError {
    match err {
        sqlx::Error::RowNotFound => DomainError::NotFound,
        other => DomainError::Internal(other.to_string()),
    }
}

pub(crate) fn parse_json_array(raw: &str) -> Vec<String> {
    serde_json::from_str(raw).unwrap_or_default()
}

pub(crate) fn dump_json_array(arr: &[String]) -> String {
    serde_json::to_string(arr).unwrap_or_else(|_| "[]".into())
}
