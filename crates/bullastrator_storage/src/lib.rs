use std::path::Path;

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

pub mod models;
pub mod repositories;

pub async fn initialize_database() -> anyhow::Result<SqlitePool> {
    let data_path = dirs_next::data_dir()
        .unwrap_or_else(|| Path::new(".").to_path_buf())
        .join("bullastrator/bullastrator.db");

    if let Some(parent) = data_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let is_new_database = !data_path.exists();
    let options = SqliteConnectOptions::new()
        .filename(&data_path)
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new().connect_with(options).await?;
    if is_new_database {
        sqlx::raw_sql(include_str!("migration/m20261004_000010_init.sql"))
            .execute(&pool)
            .await?;
    }

    Ok(pool)
}
