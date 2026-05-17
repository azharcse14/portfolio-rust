use std::sync::Arc;

use domain::{entities::Skill, ports::SkillRepository};

use crate::ApplicationResult;

pub struct ListSkills {
    repo: Arc<dyn SkillRepository>,
}

impl ListSkills {
    pub fn new(repo: Arc<dyn SkillRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> ApplicationResult<Vec<Skill>> {
        Ok(self.repo.list().await?)
    }
}
