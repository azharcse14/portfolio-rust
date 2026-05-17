use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use domain::{
    entities::Post,
    ports::PostRepository,
    value_objects::Slug,
    DomainError, DomainResult,
};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::{dump_json_array, map_sqlx, parse_json_array};

type PostRow = (
    String,         // id
    String,         // title
    String,         // slug
    String,         // content_md
    Option<String>, // cover_image
    String,         // tags
    i64,            // published
    Option<String>, // published_at
    String,         // created_at
);

pub struct SqlitePostRepository {
    pool: SqlitePool,
}

impl SqlitePostRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

fn parse_iso(raw: &str) -> DomainResult<DateTime<Utc>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(raw) {
        return Ok(dt.with_timezone(&Utc));
    }
    if let Ok(naive) = NaiveDateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S") {
        return Ok(naive.and_utc());
    }
    Err(DomainError::Internal(format!("bad datetime: {raw}")))
}

fn row_to_post(row: PostRow) -> DomainResult<Post> {
    let id =
        Uuid::parse_str(&row.0).map_err(|e| DomainError::Internal(e.to_string()))?;
    let slug = Slug::parse(row.2)?;
    let published_at = row.7.as_deref().map(parse_iso).transpose()?;
    let created_at = parse_iso(&row.8)?;

    Ok(Post {
        id,
        title: row.1,
        slug,
        content_md: row.3,
        cover_image: row.4,
        tags: parse_json_array(&row.5),
        published: row.6 != 0,
        published_at,
        created_at,
    })
}

#[async_trait]
impl PostRepository for SqlitePostRepository {
    async fn list_published(&self) -> DomainResult<Vec<Post>> {
        let rows: Vec<PostRow> = sqlx::query_as(
            "SELECT id, title, slug, content_md, cover_image, tags, published, published_at, created_at
             FROM posts WHERE published = 1 ORDER BY published_at DESC, created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;
        rows.into_iter().map(row_to_post).collect()
    }

    async fn list_all(&self) -> DomainResult<Vec<Post>> {
        let rows: Vec<PostRow> = sqlx::query_as(
            "SELECT id, title, slug, content_md, cover_image, tags, published, published_at, created_at
             FROM posts ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;
        rows.into_iter().map(row_to_post).collect()
    }

    async fn get_by_slug(&self, slug: &Slug) -> DomainResult<Post> {
        let row: PostRow = sqlx::query_as(
            "SELECT id, title, slug, content_md, cover_image, tags, published, published_at, created_at
             FROM posts WHERE slug = ?",
        )
        .bind(slug.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx)?;
        row_to_post(row)
    }

    async fn get_by_id(&self, id: Uuid) -> DomainResult<Post> {
        let row: PostRow = sqlx::query_as(
            "SELECT id, title, slug, content_md, cover_image, tags, published, published_at, created_at
             FROM posts WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx)?;
        row_to_post(row)
    }

    async fn create(&self, p: &Post) -> DomainResult<()> {
        sqlx::query(
            "INSERT INTO posts (id, title, slug, content_md, cover_image, tags, published,
                                published_at, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(p.id.to_string())
        .bind(&p.title)
        .bind(p.slug.as_str())
        .bind(&p.content_md)
        .bind(&p.cover_image)
        .bind(dump_json_array(&p.tags))
        .bind(p.published as i64)
        .bind(p.published_at.map(|d| d.to_rfc3339()))
        .bind(p.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        Ok(())
    }

    async fn update(&self, p: &Post) -> DomainResult<()> {
        sqlx::query(
            "UPDATE posts SET title = ?, slug = ?, content_md = ?, cover_image = ?, tags = ?,
                    published = ?, published_at = ?
             WHERE id = ?",
        )
        .bind(&p.title)
        .bind(p.slug.as_str())
        .bind(&p.content_md)
        .bind(&p.cover_image)
        .bind(dump_json_array(&p.tags))
        .bind(p.published as i64)
        .bind(p.published_at.map(|d| d.to_rfc3339()))
        .bind(p.id.to_string())
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        sqlx::query("DELETE FROM posts WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(map_sqlx)?;
        Ok(())
    }
}
