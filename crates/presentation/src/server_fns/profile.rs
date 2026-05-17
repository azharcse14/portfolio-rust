use domain::entities::Profile;
use leptos::prelude::*;
use leptos::server_fn::ServerFnError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProfileView {
    pub name: String,
    pub title: String,
    pub bio: String,
    pub photo_url: Option<String>,
    pub email: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
}

impl From<Profile> for ProfileView {
    fn from(p: Profile) -> Self {
        Self {
            name: p.name,
            title: p.title,
            bio: p.bio,
            photo_url: p.photo_url,
            email: p.email,
            github: p.github,
            linkedin: p.linkedin,
            twitter: p.twitter,
        }
    }
}

#[server]
pub async fn get_profile() -> Result<ProfileView, ServerFnError> {
    let state = expect_context::<crate::state::AppState>();
    let p = state.get_profile.execute().await?;
    Ok(p.into())
}
