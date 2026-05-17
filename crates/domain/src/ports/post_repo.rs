use async_trait::async_trait;
use uuid::Uuid;

use crate::{entities::Post, value_objects::Slug, DomainResult};

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn list_published(&self) -> DomainResult<Vec<Post>>;
    async fn list_all(&self) -> DomainResult<Vec<Post>>;
    async fn get_by_slug(&self, slug: &Slug) -> DomainResult<Post>;
    async fn get_by_id(&self, id: Uuid) -> DomainResult<Post>;
    async fn create(&self, post: &Post) -> DomainResult<()>;
    async fn update(&self, post: &Post) -> DomainResult<()>;
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
}
