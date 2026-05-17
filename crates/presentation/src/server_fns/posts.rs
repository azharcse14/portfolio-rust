use domain::entities::Post;
use leptos::prelude::*;
use leptos::server_fn::ServerFnError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostView {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub content_html: String,
    pub cover_image: Option<String>,
    pub tags: Vec<String>,
    pub published_at: Option<String>,
}

impl From<Post> for PostView {
    fn from(p: Post) -> Self {
        // First-paragraph excerpt — strip markdown noise lightly.
        let excerpt = p
            .content_md
            .lines()
            .find(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
            .unwrap_or("")
            .chars()
            .take(200)
            .collect();

        // Render full markdown body to HTML on the server.
        let mut html = String::new();
        let parser = pulldown_cmark::Parser::new_ext(
            &p.content_md,
            pulldown_cmark::Options::ENABLE_STRIKETHROUGH
                | pulldown_cmark::Options::ENABLE_TABLES
                | pulldown_cmark::Options::ENABLE_TASKLISTS
                | pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION,
        );
        pulldown_cmark::html::push_html(&mut html, parser);

        Self {
            id: p.id.to_string(),
            title: p.title,
            slug: p.slug.as_str().to_string(),
            excerpt,
            content_html: html,
            cover_image: p.cover_image,
            tags: p.tags,
            published_at: p.published_at.map(|d| d.format("%B %d, %Y").to_string()),
        }
    }
}

#[server]
pub async fn list_posts() -> Result<Vec<PostView>, ServerFnError> {
    let state = expect_context::<crate::state::AppState>();
    let items = state.list_posts.execute().await?;
    Ok(items.into_iter().map(Into::into).collect())
}
