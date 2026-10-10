use anyhow::{Context, Result, bail};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::Queue;

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
            sqlx::query(
                r#"
                INSERT INTO
                    queues (
                        id,
                        connection_id,
                        queue_name,
                    )
                    VALUES (?, ?, ?)
                    ON CONFLICT (connection_id, queue_name) DO NOTHING
                "#,
            )
            .bind(Uuid::new_v4().to_string())
            .bind(connection_id)
            .bind(name)
            .bind(name)
            .execute(&mut *tx)
            .await?;
        }
        if is_final_state {
            if queue_names.is_empty() {
                sqlx::query("DELETE FROM queues WHERE connection_id = ?")
                    .bind(connection_id)
                    .execute(&mut *tx)
                    .await?;
            } else {
                let placeholders = std::iter::repeat_n("?", queue_names.len())
                    .collect::<Vec<_>>()
                    .join(",");
                let query = format!(
                    "DELETE FROM queues WHERE connection_id = ? AND queue_name NOT IN ({placeholders})"
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

    pub async fn get_all_by_connection(&self, connection_id: &str) -> Result<Vec<Queue>> {
        Ok(sqlx::query_as::<_, Queue>(
            r#"
            SELECT
                id,
                connection_id,
                queue_name,
                display_name,
            FROM queues
            WHERE connection_id = ?
            ORDER BY queue_name
            "#,
        )
        .bind(connection_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch queues")?)
    }

    pub async fn get_by_name(
        &self,
        connection_id: &str,
        queue_name: &str,
    ) -> Result<Option<Queue>> {
        Ok(sqlx::query_as::<_, Queue>(
            r#"
            SELECT
                id,
                connection_id,
                queue_name,
            FROM queues
            WHERE connection_id = ?
              AND queue_name = ?
            "#,
        )
        .bind(connection_id)
        .bind(queue_name)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM queues
            WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            bail!("Queue with ID {id} not found");
        }

        Ok(result.rows_affected())
    }
}
