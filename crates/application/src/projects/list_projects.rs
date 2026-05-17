use std::sync::Arc;

use domain::{entities::Project, ports::ProjectRepository};

use crate::ApplicationResult;

pub struct ListProjects {
    repo: Arc<dyn ProjectRepository>,
}

impl ListProjects {
    pub fn new(repo: Arc<dyn ProjectRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, only_featured: bool) -> ApplicationResult<Vec<Project>> {
        Ok(self.repo.list(only_featured).await?)
    }
}
