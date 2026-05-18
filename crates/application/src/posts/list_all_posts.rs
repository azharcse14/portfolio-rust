use std::sync::Arc;

use domain::{entities::Post, ports::PostRepository};

use crate::ApplicationResult;

pub struct ListAllPosts {
    repo: Arc<dyn PostRepository>,
}

impl ListAllPosts {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> ApplicationResult<Vec<Post>> {
        Ok(self.repo.list_all().await?)
    }
}
