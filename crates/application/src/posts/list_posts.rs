use std::sync::Arc;

use domain::{entities::Post, ports::PostRepository};

use crate::ApplicationResult;

pub struct ListPublishedPosts {
    repo: Arc<dyn PostRepository>,
}

impl ListPublishedPosts {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> ApplicationResult<Vec<Post>> {
        Ok(self.repo.list_published().await?)
    }
}
