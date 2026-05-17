use std::sync::Arc;

use domain::{entities::Post, ports::PostRepository, value_objects::Slug};

use crate::ApplicationResult;

pub struct GetPost {
    repo: Arc<dyn PostRepository>,
}

impl GetPost {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, slug: &str) -> ApplicationResult<Post> {
        let slug = Slug::parse(slug)?;
        Ok(self.repo.get_by_slug(&slug).await?)
    }
}
