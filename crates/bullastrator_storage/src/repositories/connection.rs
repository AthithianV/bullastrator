use anyhow::{Context, Result, bail};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{
    Connection, ConnectionDetails, ConnectionWithQueues, CreateConnection, Queue, UpdateConnection,
    connection::{ConnectionCredentials, InsecureConnectionCredentials},
};

#[derive(sqlx::FromRow)]
struct ConnectionQueueRow {
    connection_id: String,
    workspace_id: String,
    connection_name: String,
    color: Option<String>,
    label: Option<String>,
    queue_id: Option<String>,
    queue_connection_id: Option<String>,
    queue_name: Option<String>,
}

#[derive(Clone)]
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
            r#"
                INSERT INTO
                    connections
                (
                    id,
                    workspace_id,
                    name,
                    host,
                    port,
                    password,
                    username,
                    db,
                    bullmq_prefix,
                    is_tls_enabled,
                    color, label
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(workspace_id)
        .bind(&data.name)
        .bind(&data.host)
        .bind(data.port)
        .bind(data.password)
        .bind(data.username)
        .bind(data.db.unwrap_or(0))
        .bind(data.bullmq_prefix.unwrap_or_else(|| "bull".into()))
        .bind(data.is_tls_enabled)
        .bind(data.color)
        .bind(data.label)
        .execute(&self.pool)
        .await
        .context("Failed to create connection")?;
        self.get_by_id(&id)
            .await?
            .context("Created connection was not found")
    }

    pub async fn get_all(&self, workspace_id: &str) -> Result<Vec<Connection>> {
        Ok(sqlx::query_as::<_, Connection>(
            r#"
            SELECT
                id,
                workspace_id,
                name,
                last_synced_at,
                color,
                label
            FROM
                connections
            WHERE
                workspace_id = ?
            ORDER BY
                name"#,
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn get_all_queues(&self, workspace_id: &str) -> Result<Vec<ConnectionWithQueues>> {
        let rows = sqlx::query_as::<_, ConnectionQueueRow>(
            r#"
                SELECT
                    c.id AS connection_id,
                    c.workspace_id,
                    c.name AS connection_name,
                    c.last_synced_at,
                    c.color,
                    c.label,
                    q.id AS queue_id,
                    q.connection_id AS queue_connection_id,
                    q.queue_name
                FROM connections c
                LEFT JOIN queues q ON q.connection_id = c.id
                WHERE c.workspace_id = ?
                ORDER BY c.name, q.queue_name
            "#,
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await?;

        let mut result: Vec<ConnectionWithQueues> = Vec::new();
        for row in rows {
            let connection_index =
                if let Some(index) = result.iter().position(|item| item.id == row.connection_id) {
                    index
                } else {
                    result.push(ConnectionWithQueues {
                        id: row.connection_id.clone(),
                        workspace_id: row.workspace_id,
                        name: row.connection_name,
                        color: row.color,
                        label: row.label,
                        queues: Vec::new(),
                    });
                    result.len() - 1
                };

            if let (Some(id), Some(connection_id), Some(queue_name)) =
                (row.queue_id, row.queue_connection_id, row.queue_name)
            {
                result[connection_index].queues.push(Queue {
                    id,
                    connection_id,
                    queue_name,
                });
            }
        }

        Ok(result)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Connection>> {
        Ok(sqlx::query_as::<_, Connection>(
            r#"
                    SELECT
                        id,
                        workspace_id,
                        name,
                        last_synced_at,
                        color,
                        label
                    FROM
                        connections
                    WHERE
                        id = ?
                    "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn get_details_by_id(&self, id: &str) -> Result<Option<ConnectionDetails>> {
        Ok(sqlx::query_as::<_, ConnectionDetails>(
            r#"
                    SELECT
                        id,
                        workspace_id,
                        name,
                        host,
                        port,
                        username,
                        db,
                        is_tls_enabled,
                        bullmq_prefix,
                        color,
                        label
                    FROM connections
                    WHERE id = ?
                    "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn get_credential_secure(&self, id: &str) -> Result<Option<ConnectionCredentials>> {
        Ok(sqlx::query_as::<_, ConnectionCredentials>(
            r#"
                    SELECT
                        id,
                        host,
                        port,
                        username,
                        db,
                        bullmq_prefix,
                        is_tls_enabled
                    FROM
                        connections
                    WHERE
                        id = ?
                    "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn get_credential_insecure(
        &self,
        id: &str,
    ) -> Result<Option<InsecureConnectionCredentials>> {
        Ok(sqlx::query_as::<_, InsecureConnectionCredentials>(
            r#"
                    SELECT
                        id,
                        host,
                        port,
                        password,
                        username,
                        db,
                        bullmq_prefix,
                        is_tls_enabled
                    FROM
                        connections
                    WHERE
                        id = ?
                    "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn update(&self, id: &str, data: UpdateConnection) -> Result<Connection> {
        let result = sqlx::query(
            r#"
            UPDATE
                connections
            SET
                name = COALESCE(?, name),
                host = COALESCE(?, host),
                port = COALESCE(?, port),
                username = COALESCE(?, username),
                password = COALESCE(?, password),
                db = COALESCE(?, db),
                bullmq_prefix = COALESCE(?, bullmq_prefix),
                is_tls_enabled = COALESCE(?, is_tls_enabled),
                color = COALESCE(?, color),
                label = COALESCE(?, label)
            WHERE id = ?
            "#,
        )
        .bind(data.name)
        .bind(data.host)
        .bind(data.port)
        .bind(data.username)
        .bind(data.password)
        .bind(data.db)
        .bind(data.bullmq_prefix)
        .bind(data.is_tls_enabled)
        .bind(data.color)
        .bind(data.label)
        .bind(id)
        .execute(&self.pool)
        .await?;
        if result.rows_affected() == 0 {
            bail!("Connection with ID {id} not found")
        }
        self.get_by_id(id)
            .await?
            .context("Updated connection was not found")
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        let result = sqlx::query("DELETE FROM connections WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        if result.rows_affected() == 0 {
            bail!("Connection with ID {id} not found")
        }
        Ok(result.rows_affected())
    }
}
