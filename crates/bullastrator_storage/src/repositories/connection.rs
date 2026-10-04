use anyhow::{Context, Result, bail};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{Connection, CreateConnection, UpdateConnection};

pub struct ConnectionRepository {
    pool: SqlitePool,
}

impl ConnectionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, workspace_id: &str, data: CreateConnection) -> Result<Connection> {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO connection (id, workspace_id, name, host, port, password, username, db, bullmq_prefix, is_tls_enabled, color, label) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id).bind(workspace_id).bind(&data.name).bind(&data.host).bind(data.port)
        .bind(data.password).bind(data.username).bind(data.db.unwrap_or(0))
        .bind(data.bullmq_prefix.unwrap_or_else(|| "bull".into()))
        .bind(data.is_tls_enabled).bind(data.color).bind(data.label)
        .execute(&self.pool).await.context("Failed to create connection")?;
        self.get_by_id(&id)
            .await?
            .context("Created connection was not found")
    }

    pub async fn get_all(&self, workspace_id: &str) -> Result<Vec<Connection>> {
        Ok(sqlx::query_as::<_, Connection>("SELECT id, workspace_id, name, host, port, username, password, db, is_tls_enabled, last_synced_at, bullmq_prefix, color, label, created_at FROM connection WHERE workspace_id = ? ORDER BY name")
            .bind(workspace_id).fetch_all(&self.pool).await?)
    }

    pub async fn get_all_connections(&self) -> Result<Vec<Connection>> {
        Ok(sqlx::query_as::<_, Connection>("SELECT id, workspace_id, name, host, port, username, password, db, is_tls_enabled, last_synced_at, bullmq_prefix, color, label, created_at FROM connection ORDER BY name")
            .fetch_all(&self.pool).await?)
    }

    pub async fn count_by_workspace(&self, workspace_id: &str) -> Result<u32> {
        let (count,): (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM connection WHERE workspace_id = ?")
                .bind(workspace_id)
                .fetch_one(&self.pool)
                .await?;
        Ok(count as u32)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Connection>> {
        Ok(sqlx::query_as::<_, Connection>("SELECT id, workspace_id, name, host, port, username, password, db, is_tls_enabled, last_synced_at, bullmq_prefix, color, label, created_at FROM connection WHERE id = ?").bind(id).fetch_optional(&self.pool).await?)
    }

    pub async fn get_by_id_with_password(&self, id: &str) -> Result<Option<Connection>> {
        self.get_by_id(id).await
    }

    pub async fn update(&self, id: &str, data: UpdateConnection) -> Result<Connection> {
        let result = sqlx::query("UPDATE connection SET name = COALESCE(?, name), host = COALESCE(?, host), port = COALESCE(?, port), username = COALESCE(?, username), password = COALESCE(?, password), db = COALESCE(?, db), bullmq_prefix = COALESCE(?, bullmq_prefix), is_tls_enabled = COALESCE(?, is_tls_enabled), color = COALESCE(?, color), label = COALESCE(?, label) WHERE id = ?")
            .bind(data.name).bind(data.host).bind(data.port).bind(data.username).bind(data.password).bind(data.db).bind(data.bullmq_prefix).bind(data.is_tls_enabled).bind(data.color).bind(data.label).bind(id).execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            bail!("Connection with ID {id} not found")
        }
        self.get_by_id(id)
            .await?
            .context("Updated connection was not found")
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        let result = sqlx::query("DELETE FROM connection WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        if result.rows_affected() == 0 {
            bail!("Connection with ID {id} not found")
        }
        Ok(result.rows_affected())
    }
}
