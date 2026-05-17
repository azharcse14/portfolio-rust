use async_trait::async_trait;
use domain::{entities::Skill, ports::SkillRepository, DomainError, DomainResult};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::map_sqlx;

pub struct SqliteSkillRepository {
    pool: SqlitePool,
}

impl SqliteSkillRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SkillRepository for SqliteSkillRepository {
    async fn list(&self) -> DomainResult<Vec<Skill>> {
        let rows: Vec<(String, String, Option<String>, Option<String>, i64, Option<String>, i64)> =
            sqlx::query_as(
                "SELECT id, name, icon, category, percentage, color, display_order
                 FROM skills ORDER BY display_order ASC",
            )
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx)?;

        rows.into_iter()
            .map(|(id, name, icon, category, percentage, color, display_order)| {
                Ok(Skill {
                    id: Uuid::parse_str(&id)
                        .map_err(|e| DomainError::Internal(e.to_string()))?,
                    name,
                    icon,
                    category,
                    percentage: percentage as i32,
                    color,
                    display_order: display_order as i32,
                })
            })
            .collect()
    }

    async fn create(&self, s: &Skill) -> DomainResult<()> {
        sqlx::query(
            "INSERT INTO skills (id, name, icon, category, percentage, color, display_order)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(s.id.to_string())
        .bind(&s.name)
        .bind(&s.icon)
        .bind(&s.category)
        .bind(s.percentage)
        .bind(&s.color)
        .bind(s.display_order)
        .execute(&self.pool)
        .await
        .map_err(map_sqlx)?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        sqlx::query("DELETE FROM skills WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(map_sqlx)?;
        Ok(())
    }
}
