//! Admin-only server functions. Each one calls `require_admin()` first.
//! That helper reads the `auth_token` cookie and verifies the JWT — unauthorized
//! callers get an error before any DB write happens.

use leptos::prelude::*;
use leptos::server_fn::ServerFnError;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::server_fns::auth::require_admin;

#[cfg(not(feature = "ssr"))]
async fn require_admin() -> Result<uuid::Uuid, ServerFnError> {
    Ok(uuid::Uuid::nil())
}

// ============ Profile ============

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProfileForm {
    pub name: String,
    pub title: String,
    pub bio: String,
    pub photo_url: String,
    pub email: String,
    pub github: String,
    pub linkedin: String,
    pub twitter: String,
}

#[server]
pub async fn update_profile(form: ProfileForm) -> Result<(), ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();
    let profile = domain::entities::Profile {
        name: form.name,
        title: form.title,
        bio: form.bio,
        photo_url: opt(form.photo_url),
        email: opt(form.email),
        github: opt(form.github),
        linkedin: opt(form.linkedin),
        twitter: opt(form.twitter),
    };
    state.update_profile.execute(profile).await?;
    Ok(())
}

// ============ Projects ============

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectForm {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub image_url: String,
    pub tech_stack: String, // comma-separated
    pub github_url: String,
    pub live_url: String,
    pub featured: bool,
    pub display_order: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminProjectView {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub featured: bool,
    pub display_order: i32,
}

#[server]
pub async fn list_all_projects() -> Result<Vec<AdminProjectView>, ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();
    let items = state.list_projects.execute(false).await?;
    Ok(items
        .into_iter()
        .map(|p| AdminProjectView {
            id: p.id.to_string(),
            title: p.title,
            slug: p.slug.as_str().to_string(),
            featured: p.featured,
            display_order: p.display_order,
        })
        .collect())
}

#[server]
pub async fn get_project_for_edit(slug: String) -> Result<ProjectForm, ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();
    let p = state.get_project.execute(&slug).await?;
    Ok(ProjectForm {
        title: p.title,
        slug: p.slug.as_str().to_string(),
        description: p.description,
        image_url: p.image_url.unwrap_or_default(),
        tech_stack: p.tech_stack.join(", "),
        github_url: p.github_url.unwrap_or_default(),
        live_url: p.live_url.unwrap_or_default(),
        featured: p.featured,
        display_order: p.display_order,
    })
}

#[server]
pub async fn save_project(id: String, form: ProjectForm) -> Result<(), ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();

    let dto = application::dto::NewProjectDto {
        title: form.title,
        slug: form.slug,
        description: form.description,
        image_url: opt(form.image_url),
        tech_stack: form
            .tech_stack
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        github_url: opt(form.github_url),
        live_url: opt(form.live_url),
        featured: form.featured,
        display_order: form.display_order,
    };

    if id.is_empty() {
        state.create_project.execute(dto).await?;
    } else {
        let uuid = uuid::Uuid::parse_str(&id)?;
        state.update_project.execute(uuid, dto).await?;
    }
    Ok(())
}

#[server]
pub async fn delete_project(id: String) -> Result<(), ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();
    let uuid = uuid::Uuid::parse_str(&id)?;
    state.delete_project.execute(uuid).await?;
    Ok(())
}

// ============ Posts ============

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostForm {
    pub title: String,
    pub slug: String,
    pub content_md: String,
    pub cover_image: String,
    pub tags: String, // comma-separated
    pub published: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminPostView {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub published: bool,
}

#[server]
pub async fn list_all_posts() -> Result<Vec<AdminPostView>, ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();
    let items = state.list_all_posts.execute().await?;
    Ok(items
        .into_iter()
        .map(|p| AdminPostView {
            id: p.id.to_string(),
            title: p.title,
            slug: p.slug.as_str().to_string(),
            published: p.published,
        })
        .collect())
}

#[server]
pub async fn get_post_for_edit(slug: String) -> Result<PostForm, ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();
    let p = state.get_post.execute(&slug).await?;
    Ok(PostForm {
        title: p.title,
        slug: p.slug.as_str().to_string(),
        content_md: p.content_md,
        cover_image: p.cover_image.unwrap_or_default(),
        tags: p.tags.join(", "),
        published: p.published,
    })
}

#[server]
pub async fn save_post(id: String, form: PostForm) -> Result<(), ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();

    let dto = application::dto::NewPostDto {
        title: form.title,
        slug: form.slug,
        content_md: form.content_md,
        cover_image: opt(form.cover_image),
        tags: form
            .tags
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        published: form.published,
    };

    if id.is_empty() {
        state.create_post.execute(dto).await?;
    } else {
        let uuid = uuid::Uuid::parse_str(&id)?;
        state.update_post.execute(uuid, dto).await?;
    }
    Ok(())
}

#[server]
pub async fn delete_post(id: String) -> Result<(), ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();
    let uuid = uuid::Uuid::parse_str(&id)?;
    state.delete_post.execute(uuid).await?;
    Ok(())
}

// ============ Skills ============

#[server]
pub async fn add_skill(
    name: String,
    percentage: i32,
    color: String,
    category: String,
    display_order: i32,
) -> Result<(), ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();
    state
        .create_skill
        .execute(name, percentage, opt(color), opt(category), display_order)
        .await?;
    Ok(())
}

#[server]
pub async fn delete_skill(id: String) -> Result<(), ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();
    let uuid = uuid::Uuid::parse_str(&id)?;
    state.delete_skill.execute(uuid).await?;
    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminSkillView {
    pub id: String,
    pub name: String,
    pub percentage: i32,
    pub color: String,
    pub category: String,
    pub display_order: i32,
}

#[server]
pub async fn list_all_skills() -> Result<Vec<AdminSkillView>, ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();
    let items = state.list_skills.execute().await?;
    Ok(items
        .into_iter()
        .map(|s| AdminSkillView {
            id: s.id.to_string(),
            name: s.name,
            percentage: s.percentage,
            color: s.color.unwrap_or_default(),
            category: s.category.unwrap_or_default(),
            display_order: s.display_order,
        })
        .collect())
}

// ============ Messages ============

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminMessageView {
    pub id: String,
    pub name: String,
    pub email: String,
    pub subject: String,
    pub body: String,
    pub read: bool,
    pub created_at: String,
}

#[server]
pub async fn list_messages() -> Result<Vec<AdminMessageView>, ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();
    let items = state.list_messages.execute().await?;
    Ok(items
        .into_iter()
        .map(|m| AdminMessageView {
            id: m.id.to_string(),
            name: m.name,
            email: m.email.as_str().to_string(),
            subject: m.subject.unwrap_or_default(),
            body: m.body,
            read: m.read,
            created_at: m.created_at.format("%Y-%m-%d %H:%M").to_string(),
        })
        .collect())
}

#[server]
pub async fn mark_message_read(id: String) -> Result<(), ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();
    let uuid = uuid::Uuid::parse_str(&id)?;
    state.mark_message_read.execute(uuid).await?;
    Ok(())
}

#[server]
pub async fn delete_message(id: String) -> Result<(), ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();
    let uuid = uuid::Uuid::parse_str(&id)?;
    state.delete_message.execute(uuid).await?;
    Ok(())
}

#[server]
pub async fn get_profile_form() -> Result<ProfileForm, ServerFnError> {
    require_admin().await?;
    let state = expect_context::<crate::state::AppState>();
    let p = state.get_profile.execute().await?;
    Ok(ProfileForm {
        name: p.name,
        title: p.title,
        bio: p.bio,
        photo_url: p.photo_url.unwrap_or_default(),
        email: p.email.unwrap_or_default(),
        github: p.github.unwrap_or_default(),
        linkedin: p.linkedin.unwrap_or_default(),
        twitter: p.twitter.unwrap_or_default(),
    })
}

fn opt(s: String) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
