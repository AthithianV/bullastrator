use std::collections::HashMap;

use anyhow::{Context, Result, bail};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{CreateFolder, Folder, FolderWithQueues, Queue, folder::FolderAndQueues};

#[derive(Clone)]
pub struct FolderRepository {
    pool: SqlitePool,
}

impl FolderRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, data: CreateFolder) -> Result<Folder> {
        let base = data.title.unwrap_or_else(|| "Untitled".into());

        let mut title = base.clone();
        let mut counter = 1;

        while sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM folders WHERE connection_id = ? AND title = ?",
        )
        .bind(&data.connection_id)
        .bind(&title)
        .fetch_one(&self.pool)
        .await?
            > 0
        {
            title = format!("{base} ({counter})");
            counter += 1;
        }

        let id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO folders (id, connection_id, user_id, title) VALUES (?, ?, ?, ?)")
            .bind(&id)
            .bind(&data.connection_id)
            .bind(data.user_id)
            .bind(title)
            .execute(&self.pool)
            .await?;
        self.get_by_id(&id)
            .await?
            .context("Created folder was not found")
    }

    pub async fn get_all(&self, connection_id: &str) -> Result<Vec<FolderWithQueues>> {
        self.ensure_default_folders(connection_id).await?;

        let rows = sqlx::query_as::<_, FolderAndQueues>(
            r#"
            SELECT
                f.id AS folder_id,
                f.connection_id AS folder_connection_id,
                f.user_id AS folder_user_id,
                f.title AS folder_title,

                q.id AS queue_id,
                q.connection_id AS queue_connection_id,
                q.queue_name
            FROM folders f
            LEFT JOIN folder_queues fq
                ON fq.folder_id = f.id
            LEFT JOIN queues q
                ON q.connection_id = f.connection_id
                AND (f.id = f.connection_id OR q.id = fq.queue_id)
            WHERE f.connection_id = ?
            ORDER BY
                f.created_at,
                CASE WHEN f.id = f.connection_id THEN q.queue_name END,
                CASE WHEN f.id <> f.connection_id THEN fq.sort_order END
            "#,
        )
        .bind(connection_id)
        .fetch_all(&self.pool)
        .await?;

        let mut folders: HashMap<String, FolderWithQueues> = HashMap::new();

        for row in rows {
            let folder = folders
                .entry(row.folder_id.clone())
                .or_insert_with(|| FolderWithQueues {
                    folder: Folder {
                        id: row.folder_id.clone(),
                        connection_id: row.folder_connection_id.clone(),
                        user_id: row.folder_user_id.clone(),
                        title: row.folder_title.clone(),
                    },
                    queues: Vec::new(),
                });

            if let (Some(id), Some(connection_id), Some(queue_name)) =
                (row.queue_id, row.queue_connection_id, row.queue_name)
            {
                folder.queues.push(Queue {
                    id,
                    connection_id,
                    queue_name,
                });
            }
        }

        Ok(folders.into_values().collect())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Folder>> {
        Ok(sqlx::query_as::<_, Folder>(
            "SELECT id, connection_id, user_id, title, created_at FROM folders WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn update(&self, id: &str, title: &str) -> Result<Folder> {
        let result = sqlx::query("UPDATE folders SET title = ? WHERE id = ?")
            .bind(title)
            .bind(id)
            .execute(&self.pool)
            .await?;
        if result.rows_affected() == 0 {
            bail!("Folder with ID {id} not found")
        }
        self.get_by_id(id)
            .await?
            .context("Updated folder was not found")
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        let result = sqlx::query("DELETE FROM folders WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        sqlx::query("DELETE FROM tabs WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            bail!("Folder with ID {id} not found")
        }
        Ok(result.rows_affected())
    }

    pub async fn toggle_queue_in_folder(&self, folder_id: &str, queue_id: &str) -> Result<bool> {
        let mut tx = self.pool.begin().await?;
        let existing = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM folder_queues WHERE folder_id = ? AND queue_id = ?",
        )
        .bind(folder_id)
        .bind(queue_id)
        .fetch_one(&mut *tx)
        .await?
            > 0;
        if existing {
            sqlx::query("DELETE FROM folder_queues WHERE folder_id = ? AND queue_id = ?")
                .bind(folder_id)
                .bind(queue_id)
                .execute(&mut *tx)
                .await?;
        } else {
            sqlx::query("INSERT INTO folder_queues (id, folder_id, queue_id, sort_order) VALUES (?, ?, ?, 0)").bind(Uuid::new_v4().to_string()).bind(folder_id).bind(queue_id).execute(&mut *tx).await?;
        }
        tx.commit().await?;
        Ok(!existing)
    }

    pub async fn ensure_default_folders(&self, connection_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO folders (id, connection_id, user_id, title)
            SELECT c.id, c.id, w.user_id, 'All Queues'
            FROM connections c
            LEFT JOIN workspaces w
            ON w.id = c.workspace_id
            WHERE c.id = ?
                AND NOT EXISTS (
                    SELECT 1
                    FROM folders f
                    WHERE f.id = c.id
                )
            "#,
        )
        .bind(connection_id)
        .bind(connection_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn reorder_queues(
        &self,
        folder_id: &str,
        ordered_queue_ids: &[String],
    ) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        for (index, queue_id) in ordered_queue_ids.iter().enumerate() {
            sqlx::query(
                "UPDATE folder_queues SET sort_order = ? WHERE folder_id = ? AND queue_id = ?",
            )
            .bind(index as i32)
            .bind(folder_id)
            .bind(queue_id)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await.context("Failed to commit queue reorder")
    }

    pub async fn get_queues_for_folder(&self, folder_id: &str) -> Result<Vec<Queue>> {
        Ok(sqlx::query_as::<_, Queue>(
            r#"
                SELECT
                    q.id,
                    q.connection_id,
                    q.queue_name
                FROM folders f
                JOIN queues q ON q.connection_id = f.connection_id
                LEFT JOIN folder_queues fq
                    ON fq.folder_id = f.id AND fq.queue_id = q.id
                WHERE f.id = ?
                  AND (f.id = f.connection_id OR fq.queue_id IS NOT NULL)
                ORDER BY
                    CASE WHEN f.id = f.connection_id THEN q.queue_name END,
                    CASE WHEN f.id <> f.connection_id THEN fq.sort_order END
            "#,
        )
        .bind(folder_id)
        .fetch_all(&self.pool)
        .await?)
    }
}
