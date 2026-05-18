use std::sync::Arc;

use chrono::Utc;
use domain::{ports::PostRepository, value_objects::Slug};
use uuid::Uuid;

use crate::{dto::NewPostDto, ApplicationResult};

pub struct UpdatePost {
    repo: Arc<dyn PostRepository>,
}

impl UpdatePost {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid, dto: NewPostDto) -> ApplicationResult<()> {
        let mut p = self.repo.get_by_id(id).await?;
        let was_published = p.published;
        p.title = dto.title;
        p.slug = Slug::parse(dto.slug)?;
        p.content_md = dto.content_md;
        p.cover_image = dto.cover_image;
        p.tags = dto.tags;
        p.published = dto.published;
        // First publish stamps the publish time; later edits don't reset it.
        if !was_published && dto.published && p.published_at.is_none() {
            p.published_at = Some(Utc::now());
        }
        if !dto.published {
            p.published_at = None;
        }
        self.repo.update(&p).await?;
        Ok(())
    }
}
