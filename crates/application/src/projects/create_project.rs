use std::sync::Arc;

use chrono::Utc;
use domain::{entities::Project, ports::ProjectRepository, value_objects::Slug};
use uuid::Uuid;

use crate::{dto::NewProjectDto, ApplicationResult};

pub struct CreateProject {
    repo: Arc<dyn ProjectRepository>,
}

impl CreateProject {
    pub fn new(repo: Arc<dyn ProjectRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, dto: NewProjectDto) -> ApplicationResult<Uuid> {
        let p = Project {
            id: Uuid::new_v4(),
            title: dto.title,
            slug: Slug::parse(dto.slug)?,
            description: dto.description,
            image_url: dto.image_url,
            tech_stack: dto.tech_stack,
            github_url: dto.github_url,
            live_url: dto.live_url,
            featured: dto.featured,
            display_order: dto.display_order,
            created_at: Utc::now(),
        };
        let id = p.id;
        self.repo.create(&p).await?;
        Ok(id)
    }
}
