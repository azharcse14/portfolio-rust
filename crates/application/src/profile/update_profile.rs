use std::sync::Arc;

use domain::{entities::Profile, ports::ProfileRepository};

use crate::ApplicationResult;

pub struct UpdateProfile {
    repo: Arc<dyn ProfileRepository>,
}

impl UpdateProfile {
    pub fn new(repo: Arc<dyn ProfileRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, p: Profile) -> ApplicationResult<()> {
        self.repo.upsert(&p).await?;
        Ok(())
    }
}
