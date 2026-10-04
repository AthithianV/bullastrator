use anyhow::{Context, Result, bail};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{Connection, ConnectionWithQueues, Queue, UpdateQueue};

#[derive(Clone)]
pub struct QueueRepository {
    pool: SqlitePool,
}

impl QueueRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn sync_all_queues(
        &self,
        connection_id: &str,
        queue_names: &[String],
        is_final_state: bool,
    ) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        for name in queue_names {
            sqlx::query("INSERT INTO queue (id, connection_id, queue_name, display_name, is_starred, auto_refresh_rate, notification_settings) VALUES (?, ?, ?, ?, 0, 5000, 'critical') ON CONFLICT (connection_id, queue_name) DO NOTHING")
                .bind(Uuid::new_v4().to_string()).bind(connection_id).bind(name).bind(name).execute(&mut *tx).await?;
        }
        if is_final_state {
            if queue_names.is_empty() {
                sqlx::query("DELETE FROM queue WHERE connection_id = ?")
                    .bind(connection_id)
                    .execute(&mut *tx)
                    .await?;
            } else {
                let placeholders = std::iter::repeat_n("?", queue_names.len())
                    .collect::<Vec<_>>()
                    .join(",");
                let query = format!(
                    "DELETE FROM queue WHERE connection_id = ? AND queue_name NOT IN ({placeholders})"
                );
                let mut request = sqlx::query(&query).bind(connection_id);
                for name in queue_names {
                    request = request.bind(name);
                }
                request.execute(&mut *tx).await?;
            }
        }
        tx.commit().await?;
        Ok(())
    }

    pub async fn get_all_by_workspace(
        &self,
        workspace_id: &str,
    ) -> Result<Vec<ConnectionWithQueues>> {
        let connections = sqlx::query_as::<_, Connection>("SELECT id, workspace_id, name, host, port, password, username, db, last_synced_at, bullmq_prefix, is_tls_enabled, color, label, created_at FROM connection WHERE workspace_id = ? ORDER BY name").bind(workspace_id).fetch_all(&self.pool).await?;
        let mut result = Vec::with_capacity(connections.len());
        for connection in connections {
            let queues = self.get_all_by_connection(&connection.id).await?;
            result.push(ConnectionWithQueues { connection, queues });
        }
        Ok(result)
    }

    pub async fn get_all_by_connection(&self, connection_id: &str) -> Result<Vec<Queue>> {
        Ok(sqlx::query_as::<_, Queue>("SELECT id, connection_id, queue_name, display_name, is_starred, auto_refresh_rate, notification_settings, created_at FROM queue WHERE connection_id = ? ORDER BY queue_name").bind(connection_id).fetch_all(&self.pool).await.context("Failed to fetch queues")?)
    }

    pub async fn get_by_name(
        &self,
        connection_id: &str,
        queue_name: &str,
    ) -> Result<Option<Queue>> {
        Ok(sqlx::query_as::<_, Queue>("SELECT id, connection_id, queue_name, display_name, is_starred, auto_refresh_rate, notification_settings, created_at FROM queue WHERE connection_id = ? AND queue_name = ?").bind(connection_id).bind(queue_name).fetch_optional(&self.pool).await?)
    }

    pub async fn update(&self, id: &str, data: UpdateQueue) -> Result<Queue> {
        let result = sqlx::query("UPDATE queue SET display_name = COALESCE(?, display_name), is_starred = COALESCE(?, is_starred), auto_refresh_rate = COALESCE(?, auto_refresh_rate), notification_settings = COALESCE(?, notification_settings) WHERE id = ?").bind(data.display_name).bind(data.is_starred).bind(data.auto_refresh_rate).bind(data.notification_settings).bind(id).execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            bail!("Queue with ID {id} not found")
        }
        Ok(sqlx::query_as::<_, Queue>("SELECT id, connection_id, queue_name, display_name, is_starred, auto_refresh_rate, notification_settings, created_at FROM queue WHERE id = ?").bind(id).fetch_one(&self.pool).await?)
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        let result = sqlx::query("DELETE FROM queue WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        if result.rows_affected() == 0 {
            bail!("Queue with ID {id} not found")
        }
        Ok(result.rows_affected())
    }
}
