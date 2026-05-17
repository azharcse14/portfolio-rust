use std::sync::Arc;

use chrono::Utc;
use domain::{entities::Message, ports::MessageRepository, value_objects::Email};
use uuid::Uuid;

use crate::{dto::ContactMessageDto, ApplicationError, ApplicationResult};

pub struct SubmitMessage {
    repo: Arc<dyn MessageRepository>,
}

impl SubmitMessage {
    pub fn new(repo: Arc<dyn MessageRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, dto: ContactMessageDto) -> ApplicationResult<()> {
        let name = dto.name.trim();
        let body = dto.body.trim();
        if name.is_empty() {
            return Err(ApplicationError::Invalid("name is required".into()));
        }
        if body.is_empty() {
            return Err(ApplicationError::Invalid("message body is required".into()));
        }

        let msg = Message {
            id: Uuid::new_v4(),
            name: name.to_string(),
            email: Email::parse(dto.email)?,
            subject: dto.subject.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
            body: body.to_string(),
            read: false,
            created_at: Utc::now(),
        };

        self.repo.create(&msg).await?;
        Ok(())
    }
}
