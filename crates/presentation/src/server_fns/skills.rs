use domain::entities::Skill;
use leptos::prelude::*;
use leptos::server_fn::ServerFnError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillView {
    pub name: String,
    pub percentage: i32,
    pub color: String,
}

impl From<Skill> for SkillView {
    fn from(s: Skill) -> Self {
        Self {
            name: s.name,
            percentage: s.percentage,
            color: s.color.unwrap_or_else(|| "color-1".into()),
        }
    }
}

#[server]
pub async fn list_skills() -> Result<Vec<SkillView>, ServerFnError> {
    let state = expect_context::<crate::state::AppState>();
    let items = state.list_skills.execute().await?;
    Ok(items.into_iter().map(Into::into).collect())
}
