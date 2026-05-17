use async_trait::async_trait;

use crate::{value_objects::Email, DomainResult};

#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send(&self, to: &Email, subject: &str, body: &str) -> DomainResult<()>;
}
