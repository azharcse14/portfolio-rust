use std::sync::Arc;

use domain::{ports::ProjectRepository, value_objects::Slug};
use uuid::Uuid;

use crate::{dto::NewProjectDto, ApplicationResult};

pub struct UpdateProject {
    repo: Arc<dyn ProjectRepository>,
}

impl UpdateProject {
    pub fn new(repo: Arc<dyn ProjectRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid, dto: NewProjectDto) -> ApplicationResult<()> {
        let mut p = self.repo.get_by_id(id).await?;
        p.title = dto.title;
        p.slug = Slug::parse(dto.slug)?;
        p.description = dto.description;
        p.image_url = dto.image_url;
        p.tech_stack = dto.tech_stack;
        p.github_url = dto.github_url;
        p.live_url = dto.live_url;
        p.featured = dto.featured;
        p.display_order = dto.display_order;
        self.repo.update(&p).await?;
        Ok(())
    }
}
