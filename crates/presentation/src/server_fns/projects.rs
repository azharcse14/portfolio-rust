use domain::entities::Project;
use leptos::prelude::*;
use leptos::server_fn::ServerFnError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectView {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub image_url: Option<String>,
    pub tech_stack: Vec<String>,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub featured: bool,
}

impl From<Project> for ProjectView {
    fn from(p: Project) -> Self {
        Self {
            id: p.id.to_string(),
            title: p.title,
            slug: p.slug.as_str().to_string(),
            description: p.description,
            image_url: p.image_url,
            tech_stack: p.tech_stack,
            github_url: p.github_url,
            live_url: p.live_url,
            featured: p.featured,
        }
    }
}

#[server]
pub async fn list_projects(only_featured: bool) -> Result<Vec<ProjectView>, ServerFnError> {
    let state = expect_context::<crate::state::AppState>();
    let items = state.list_projects.execute(only_featured).await?;
    Ok(items.into_iter().map(Into::into).collect())
}
