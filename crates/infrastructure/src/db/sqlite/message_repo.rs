use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use domain::{
    entities::Message,
    ports::MessageRepository,
    value_objects::Email,
    DomainError, DomainResult,
};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::map_sqlx;

type MessageRow = (
    String,         // id
    String,         // name
    String,         // email
    Option<String>, // subject
    String,         // body
    i64,            // read
    String,         // created_at
);

pub struct SqliteMessageRepository {
    pool: SqlitePool,
}

impl SqliteMessageRepository {
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

fn row_to_message(row: MessageRow) -> DomainResult<Message> {
    let id =
        Uuid::parse_str(&row.0).map_err(|e| DomainError::Internal(e.to_string()))?;
    let email = Email::parse(row.2)?;
    let created_at = parse_iso(&row.6)?;
    Ok(Message {
        id,
        name: row.1,
        email,
        subject: row.3,
        body: row.4,
        read: row.5 != 0,
        created_at,
    })
}

#[async_trait]
impl MessageRepository for SqliteMessageRepository {
    async fn list(&self) -> DomainResult<Vec<Message>> {
        let rows: Vec<MessageRow> = sqlx::query_as(
            "SELECT id, name, email, subject, body, read, created_at
             FROM messages ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx)?;
        rows.into_iter().map(row_to_message).collect()
    }

    async fn get(&self, id: Uuid) -> DomainResult<Message> {
        let row: MessageRow = sqlx::query_as(
            "SELECT id, name, email, subject, body, read, created_at
             FROM messages WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(map_sqlx)?;
        row_to_message(row)
    }

    async fn create(&self, m: &Message) -> DomainResult<()> {
        sqlx::query(
            "INSERT INTO messages (id, name, email, subject, body, read, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(m.id.to_string())
        .bind(&m.name)
        .bind(m.email.as_str())
        .bind(&m.subject)
        .bind(&m.body)
        .bind(m.read as i64)
        .bind(m.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        Ok(())
    }

    async fn mark_read(&self, id: Uuid) -> DomainResult<()> {
        sqlx::query("UPDATE messages SET read = 1 WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(map_sqlx)?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        sqlx::query("DELETE FROM messages WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(map_sqlx)?;
        Ok(())
    }

    async fn unread_count(&self) -> DomainResult<u64> {
        let (n,): (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM messages WHERE read = 0")
                .fetch_one(&self.pool)
                .await
                .map_err(map_sqlx)?;
        Ok(n as u64)
    }
}
