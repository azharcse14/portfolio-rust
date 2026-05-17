//! Data Transfer Objects — boundary types between presentation and application.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewProjectDto {
    pub title: String,
    pub slug: String,
    pub description: String,
    pub image_url: Option<String>,
    pub tech_stack: Vec<String>,
    pub github_url: Option<String>,
    pub live_url: Option<String>,
    pub featured: bool,
    pub display_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPostDto {
    pub title: String,
    pub slug: String,
    pub content_md: String,
    pub cover_image: Option<String>,
    pub tags: Vec<String>,
    pub published: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactMessageDto {
    pub name: String,
    pub email: String,
    pub subject: Option<String>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}
