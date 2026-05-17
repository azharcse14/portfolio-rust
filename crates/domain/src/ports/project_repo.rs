use async_trait::async_trait;
use uuid::Uuid;

use crate::{entities::Project, value_objects::Slug, DomainResult};

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn list(&self, only_featured: bool) -> DomainResult<Vec<Project>>;
    async fn get_by_slug(&self, slug: &Slug) -> DomainResult<Project>;
    async fn get_by_id(&self, id: Uuid) -> DomainResult<Project>;
    async fn create(&self, project: &Project) -> DomainResult<()>;
    async fn update(&self, project: &Project) -> DomainResult<()>;
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
}
