//! Domain layer — pure business types and ports (traits).
//! No I/O, no framework, no DB. Depends on nothing else in the workspace.

pub mod entities;
pub mod errors;
pub mod ports;
pub mod value_objects;

pub use errors::DomainError;
pub type DomainResult<T> = Result<T, DomainError>;
