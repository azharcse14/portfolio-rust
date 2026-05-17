use std::sync::Arc;

use domain::{entities::Project, ports::ProjectRepository, value_objects::Slug};

use crate::ApplicationResult;

pub struct GetProject {
    repo: Arc<dyn ProjectRepository>,
}

impl GetProject {
    pub fn new(repo: Arc<dyn ProjectRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, slug: &str) -> ApplicationResult<Project> {
        let slug = Slug::parse(slug)?;
        Ok(self.repo.get_by_slug(&slug).await?)
    }
}
