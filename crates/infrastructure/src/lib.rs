//! Infrastructure layer — concrete adapters that implement `domain::ports`.
//! Holds DB, email, image-storage, auth crypto, and config plumbing.

pub mod auth;
pub mod config;
pub mod db;
pub mod email;
pub mod storage;

pub use config::AppConfig;
pub use db::{sqlite, Database};
