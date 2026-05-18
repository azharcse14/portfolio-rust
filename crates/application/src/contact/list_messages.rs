use std::sync::Arc;

use domain::{entities::Message, ports::MessageRepository};

use crate::ApplicationResult;

pub struct ListMessages {
    repo: Arc<dyn MessageRepository>,
}

impl ListMessages {
    pub fn new(repo: Arc<dyn MessageRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> ApplicationResult<Vec<Message>> {
        Ok(self.repo.list().await?)
    }
}
