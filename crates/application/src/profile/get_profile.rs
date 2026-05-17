use std::sync::Arc;

use domain::{entities::Profile, ports::ProfileRepository};

use crate::ApplicationResult;

pub struct GetProfile {
    repo: Arc<dyn ProfileRepository>,
}

impl GetProfile {
    pub fn new(repo: Arc<dyn ProfileRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> ApplicationResult<Profile> {
        Ok(self.repo.get().await?)
    }
}
