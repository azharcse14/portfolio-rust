use async_trait::async_trait;

use crate::{entities::Profile, DomainResult};

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn get(&self) -> DomainResult<Profile>;
    async fn upsert(&self, profile: &Profile) -> DomainResult<()>;
}
