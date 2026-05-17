use async_trait::async_trait;
use domain::{entities::Profile, ports::ProfileRepository, DomainResult};
use sqlx::SqlitePool;

use super::map_sqlx;

pub struct SqliteProfileRepository {
    pool: SqlitePool,
}

impl SqliteProfileRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProfileRepository for SqliteProfileRepository {
    async fn get(&self) -> DomainResult<Profile> {
        let row: (String, String, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>) =
            sqlx::query_as(
                "SELECT name, title, bio, photo_url, email, github, linkedin, twitter FROM profile WHERE id = 1",
            )
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx)?;

        Ok(Profile {
            name: row.0,
            title: row.1,
            bio: row.2,
            photo_url: row.3,
            email: row.4,
            github: row.5,
            linkedin: row.6,
            twitter: row.7,
        })
    }

    async fn upsert(&self, p: &Profile) -> DomainResult<()> {
        sqlx::query(
            r#"
            INSERT INTO profile (id, name, title, bio, photo_url, email, github, linkedin, twitter)
            VALUES (1, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                title = excluded.title,
                bio = excluded.bio,
                photo_url = excluded.photo_url,
                email = excluded.email,
                github = excluded.github,
                linkedin = excluded.linkedin,
                twitter = excluded.twitter
            "#,
        )
        .bind(&p.name)
        .bind(&p.title)
        .bind(&p.bio)
        .bind(&p.photo_url)
        .bind(&p.email)
        .bind(&p.github)
        .bind(&p.linkedin)
        .bind(&p.twitter)
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;

        Ok(())
    }
}
