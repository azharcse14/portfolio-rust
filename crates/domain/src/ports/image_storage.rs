use async_trait::async_trait;

use crate::DomainResult;

#[async_trait]
pub trait ImageStorage: Send + Sync {
    async fn upload(&self, filename: &str, bytes: &[u8]) -> DomainResult<String>;
    async fn delete(&self, public_id: &str) -> DomainResult<()>;
}
