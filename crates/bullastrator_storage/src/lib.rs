use std::path::Path;

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::migration::MIGRATION_TABLE;

mod migration;

pub mod models;
pub mod repositories;

pub async fn initialize_database() -> anyhow::Result<SqlitePool> {
    let data_path = dirs_next::data_dir()
        .unwrap_or_else(|| Path::new(".").to_path_buf())
        .join("bullastrator/bullastrator.db");

    if let Some(parent) = data_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let options = SqliteConnectOptions::new()
        .filename(&data_path)
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new().connect_with(options).await?;

    // The migration runner needs this table before it can check whether the
    // first migration has already been applied.
    sqlx::raw_sql(MIGRATION_TABLE.sql).execute(&pool).await?;

    for migration in migration::MIGRATIONS {
        let already_applied: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM schema_migrations WHERE version = ?)")
                .bind(migration.version)
                .fetch_one(&pool)
                .await?;

        if already_applied {
            continue;
        }

        let mut transaction = pool.begin().await?;
        sqlx::raw_sql(migration.sql)
            .execute(&mut *transaction)
            .await?;
        sqlx::query("INSERT INTO schema_migrations (version) VALUES (?)")
            .bind(migration.version)
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
    }

    Ok(pool)
}
