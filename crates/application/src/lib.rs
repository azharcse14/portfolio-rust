//! Application layer — use-cases. Orchestrates domain entities via ports.
//! Depends only on `domain`. No I/O, no framework, no SQL.

pub mod auth;
pub mod contact;
pub mod dto;
pub mod error;
pub mod posts;
pub mod profile;
pub mod projects;
pub mod skills;

pub use error::ApplicationError;
pub type ApplicationResult<T> = Result<T, ApplicationError>;
