use std::sync::Arc;

use domain::{entities::Skill, ports::SkillRepository};
use uuid::Uuid;

use crate::ApplicationResult;

pub struct CreateSkill {
    repo: Arc<dyn SkillRepository>,
}

impl CreateSkill {
    pub fn new(repo: Arc<dyn SkillRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        name: String,
        percentage: i32,
        color: Option<String>,
        category: Option<String>,
        display_order: i32,
    ) -> ApplicationResult<Uuid> {
        let s = Skill {
            id: Uuid::new_v4(),
            name,
            icon: None,
            category,
            percentage,
            color,
            display_order,
        };
        let id = s.id;
        self.repo.create(&s).await?;
        Ok(id)
    }
}
