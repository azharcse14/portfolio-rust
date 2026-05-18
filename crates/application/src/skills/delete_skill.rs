use std::sync::Arc;

use domain::ports::SkillRepository;
use uuid::Uuid;

use crate::ApplicationResult;

pub struct DeleteSkill {
    repo: Arc<dyn SkillRepository>,
}

impl DeleteSkill {
    pub fn new(repo: Arc<dyn SkillRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, id: Uuid) -> ApplicationResult<()> {
        self.repo.delete(id).await?;
        Ok(())
    }
}
