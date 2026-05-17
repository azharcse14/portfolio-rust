use async_trait::async_trait;
use uuid::Uuid;

use crate::{entities::User, value_objects::Email, DomainResult};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &Email) -> DomainResult<User>;
    async fn find_by_id(&self, id: Uuid) -> DomainResult<User>;
    async fn create(&self, user: &User) -> DomainResult<()>;
    async fn count(&self) -> DomainResult<u64>;
}
