use std::sync::Arc;

use chrono::Utc;
use domain::{entities::Post, ports::PostRepository, value_objects::Slug};
use uuid::Uuid;

use crate::{dto::NewPostDto, ApplicationResult};

pub struct CreatePost {
    repo: Arc<dyn PostRepository>,
}

impl CreatePost {
    pub fn new(repo: Arc<dyn PostRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, dto: NewPostDto) -> ApplicationResult<Uuid> {
        let now = Utc::now();
        let p = Post {
            id: Uuid::new_v4(),
            title: dto.title,
            slug: Slug::parse(dto.slug)?,
            content_md: dto.content_md,
            cover_image: dto.cover_image,
            tags: dto.tags,
            published: dto.published,
            published_at: if dto.published { Some(now) } else { None },
            created_at: now,
        };
        let id = p.id;
        self.repo.create(&p).await?;
        Ok(id)
    }
}
