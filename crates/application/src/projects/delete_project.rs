use std::sync::Arc;

use domain::ports::ProjectRepository;
use uuid::Uuid;

use crate::ApplicationResult;

pub struct DeleteProject {
    repo: Arc<dyn ProjectRepository>,
}

impl DeleteProject {
    pub fn new(repo: Arc<dyn ProjectRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> ApplicationResult<()> {
        self.repo.delete(id).await?;
        Ok(())
    }
}
