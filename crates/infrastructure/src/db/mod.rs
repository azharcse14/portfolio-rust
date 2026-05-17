pub mod sqlite;

use sqlx::SqlitePool;

pub type Database = SqlitePool;

/// Open a SQLite pool at the given URL and run all bundled migrations.
pub async fn connect_and_migrate(database_url: &str) -> anyhow::Result<Database> {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(8)
        .connect(database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
