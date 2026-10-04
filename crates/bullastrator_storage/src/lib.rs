pub mod models;
pub mod repositories;

pub async fn initialize_database(pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    sqlx::raw_sql(include_str!(
        "migration/m20261004_000010_add_user_access.sql"
    ))
    .execute(pool)
    .await?;
    Ok(())
}
