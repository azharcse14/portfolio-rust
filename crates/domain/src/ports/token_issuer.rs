use uuid::Uuid;

use crate::DomainResult;

pub trait TokenIssuer: Send + Sync {
    /// Issue a signed token carrying the user id, valid for `ttl_hours`.
    fn issue(&self, user_id: Uuid, ttl_hours: i64) -> DomainResult<String>;

    /// Verify the token and return the user id it carries.
    fn verify(&self, token: &str) -> DomainResult<Uuid>;
}
