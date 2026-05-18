use std::sync::Arc;

use domain::ports::MessageRepository;
use uuid::Uuid;

use crate::ApplicationResult;

pub struct MarkMessageRead {
    repo: Arc<dyn MessageRepository>,
}

impl MarkMessageRead {
    pub fn new(repo: Arc<dyn MessageRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> ApplicationResult<()> {
        self.repo.mark_read(id).await?;
        Ok(())
    }
}
