use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use domain::{
    entities::User,
    ports::UserRepository,
    value_objects::Email,
    DomainError, DomainResult,
};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::map_sqlx;

type UserRow = (String, String, String, Option<String>, String);

pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
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

fn row_to_user(row: UserRow) -> DomainResult<User> {
    let id =
        Uuid::parse_str(&row.0).map_err(|e| DomainError::Internal(e.to_string()))?;
    let email = Email::parse(row.1)?;
    let created_at = parse_iso(&row.4)?;
    Ok(User {
        id,
        email,
        password_hash: row.2,
        name: row.3,
        created_at,
    })
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn find_by_email(&self, email: &Email) -> DomainResult<User> {
        let row: UserRow = sqlx::query_as(
            "SELECT id, email, password_hash, name, created_at FROM users WHERE email = ?",
        )
        .bind(email.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx)?;
        row_to_user(row)
    }

    async fn find_by_id(&self, id: Uuid) -> DomainResult<User> {
        let row: UserRow = sqlx::query_as(
            "SELECT id, email, password_hash, name, created_at FROM users WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx)?;
        row_to_user(row)
    }

    async fn create(&self, u: &User) -> DomainResult<()> {
        sqlx::query(
            "INSERT INTO users (id, email, password_hash, name, created_at)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(u.id.to_string())
        .bind(u.email.as_str())
        .bind(&u.password_hash)
        .bind(&u.name)
        .bind(u.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        Ok(())
    }

    async fn count(&self) -> DomainResult<u64> {
        let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx)?;
        Ok(n as u64)
    }
}
