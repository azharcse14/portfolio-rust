use std::sync::Arc;

use domain::ports::MessageRepository;
use uuid::Uuid;

use crate::ApplicationResult;

pub struct DeleteMessage {
    repo: Arc<dyn MessageRepository>,
}

impl DeleteMessage {
    pub fn new(repo: Arc<dyn MessageRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> ApplicationResult<()> {
        self.repo.delete(id).await?;
        Ok(())
    }
}
