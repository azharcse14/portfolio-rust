use std::sync::Arc;

use domain::ports::PostRepository;
use uuid::Uuid;

use crate::ApplicationResult;

pub struct DeletePost {
    repo: Arc<dyn PostRepository>,
}

impl DeletePost {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> ApplicationResult<()> {
        self.repo.delete(id).await?;
        Ok(())
    }
}
