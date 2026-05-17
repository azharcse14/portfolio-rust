use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use domain::{
    entities::Project,
    ports::ProjectRepository,
    value_objects::Slug,
    DomainError, DomainResult,
};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::{dump_json_array, map_sqlx, parse_json_array};

type ProjectRow = (
    String,         // id
    String,         // title
    String,         // slug
    String,         // description
    Option<String>, // image_url
    String,         // tech_stack (JSON)
    Option<String>, // github_url
    Option<String>, // live_url
    i64,            // featured
    i64,            // display_order
    String,         // created_at
);

pub struct SqliteProjectRepository {
    pool: SqlitePool,
}

impl SqliteProjectRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

fn row_to_project(row: ProjectRow) -> DomainResult<Project> {
    let id =
        Uuid::parse_str(&row.0).map_err(|e| DomainError::Internal(e.to_string()))?;
    let slug = Slug::parse(row.2)?;
    let created_at = parse_iso(&row.10)?;
    Ok(Project {
        id,
        title: row.1,
        slug,
        description: row.3,
        image_url: row.4,
        tech_stack: parse_json_array(&row.5),
        github_url: row.6,
        live_url: row.7,
        featured: row.8 != 0,
        display_order: row.9 as i32,
        created_at,
    })
}

fn parse_iso(raw: &str) -> DomainResult<DateTime<Utc>> {
    // SQLite's datetime('now') yields "YYYY-MM-DD HH:MM:SS" (no tz, UTC).
    if let Ok(dt) = DateTime::parse_from_rfc3339(raw) {
        return Ok(dt.with_timezone(&Utc));
    }
    if let Ok(naive) = NaiveDateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S") {
        return Ok(naive.and_utc());
    }
    Err(DomainError::Internal(format!("bad datetime: {raw}")))
}

#[async_trait]
impl ProjectRepository for SqliteProjectRepository {
    async fn list(&self, only_featured: bool) -> DomainResult<Vec<Project>> {
        let sql = if only_featured {
            "SELECT id, title, slug, description, image_url, tech_stack, github_url, live_url,
                    featured, display_order, created_at
             FROM projects WHERE featured = 1 ORDER BY display_order ASC"
        } else {
            "SELECT id, title, slug, description, image_url, tech_stack, github_url, live_url,
                    featured, display_order, created_at
             FROM projects ORDER BY display_order ASC"
        };
        let rows: Vec<ProjectRow> = sqlx::query_as(sql)
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx)?;
        rows.into_iter().map(row_to_project).collect()
    }

    async fn get_by_slug(&self, slug: &Slug) -> DomainResult<Project> {
        let row: ProjectRow = sqlx::query_as(
            "SELECT id, title, slug, description, image_url, tech_stack, github_url, live_url,
                    featured, display_order, created_at
             FROM projects WHERE slug = ?",
        )
        .bind(slug.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx)?;
        row_to_project(row)
    }

    async fn get_by_id(&self, id: Uuid) -> DomainResult<Project> {
        let row: ProjectRow = sqlx::query_as(
            "SELECT id, title, slug, description, image_url, tech_stack, github_url, live_url,
                    featured, display_order, created_at
             FROM projects WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx)?;
        row_to_project(row)
    }

    async fn create(&self, p: &Project) -> DomainResult<()> {
        sqlx::query(
            "INSERT INTO projects (id, title, slug, description, image_url, tech_stack,
                                   github_url, live_url, featured, display_order, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(p.id.to_string())
        .bind(&p.title)
        .bind(p.slug.as_str())
        .bind(&p.description)
        .bind(&p.image_url)
        .bind(dump_json_array(&p.tech_stack))
        .bind(&p.github_url)
        .bind(&p.live_url)
        .bind(p.featured as i64)
        .bind(p.display_order as i64)
        .bind(p.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        Ok(())
    }

    async fn update(&self, p: &Project) -> DomainResult<()> {
        sqlx::query(
            "UPDATE projects SET title = ?, slug = ?, description = ?, image_url = ?,
                    tech_stack = ?, github_url = ?, live_url = ?, featured = ?, display_order = ?
             WHERE id = ?",
        )
        .bind(&p.title)
        .bind(p.slug.as_str())
        .bind(&p.description)
        .bind(&p.image_url)
        .bind(dump_json_array(&p.tech_stack))
        .bind(&p.github_url)
        .bind(&p.live_url)
        .bind(p.featured as i64)
        .bind(p.display_order as i64)
        .bind(p.id.to_string())
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        sqlx::query("DELETE FROM projects WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(map_sqlx)?;
        Ok(())
    }
}
