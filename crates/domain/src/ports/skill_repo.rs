use async_trait::async_trait;
use uuid::Uuid;

use crate::{entities::Skill, DomainResult};

#[async_trait]
pub trait SkillRepository: Send + Sync {
    async fn list(&self) -> DomainResult<Vec<Skill>>;
    async fn create(&self, skill: &Skill) -> DomainResult<()>;
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
}
