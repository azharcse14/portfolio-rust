use async_trait::async_trait;
use uuid::Uuid;

use crate::{entities::Message, DomainResult};

#[async_trait]
pub trait MessageRepository: Send + Sync {
    async fn list(&self) -> DomainResult<Vec<Message>>;
    async fn get(&self, id: Uuid) -> DomainResult<Message>;
    async fn create(&self, msg: &Message) -> DomainResult<()>;
    async fn mark_read(&self, id: Uuid) -> DomainResult<()>;
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
    async fn unread_count(&self) -> DomainResult<u64>;
}
