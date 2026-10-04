use anyhow::{bail, Context, Result};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{CreateFolder, Folder, FolderWithQueues, Queue};

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
            "SELECT COUNT(*) FROM folder WHERE connection_id = ? AND title = ?",
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
        sqlx::query("INSERT INTO folder (id, connection_id, user_id, title) VALUES (?, ?, ?, ?)")
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
        let folders = sqlx::query_as::<_, Folder>("SELECT id, connection_id, user_id, title, created_at FROM folder WHERE connection_id = ? ORDER BY created_at")
            .bind(connection_id).fetch_all(&self.pool).await?;
        let mut result = Vec::with_capacity(folders.len());
        for folder in folders {
            let queues = sqlx::query_as::<_, Queue>("SELECT q.id, q.connection_id, q.queue_name, q.display_name, q.is_starred, q.auto_refresh_rate, q.notification_settings, q.created_at FROM queue q JOIN folder_queue fq ON fq.queue_id = q.id WHERE fq.folder_id = ? ORDER BY fq.sort_order")
                .bind(&folder.id).fetch_all(&self.pool).await?;
            result.push(FolderWithQueues { folder, queues });
        }
        Ok(result)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Folder>> {
        Ok(sqlx::query_as::<_, Folder>(
            "SELECT id, connection_id, user_id, title, created_at FROM folder WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn update(&self, id: &str, title: &str) -> Result<Folder> {
        let result = sqlx::query("UPDATE folder SET title = ? WHERE id = ?")
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
        let result = sqlx::query("DELETE FROM folder WHERE id = ?")
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
            "SELECT COUNT(*) FROM folder_queue WHERE folder_id = ? AND queue_id = ?",
        )
        .bind(folder_id)
        .bind(queue_id)
        .fetch_one(&mut *tx)
        .await?
            > 0;
        if existing {
            sqlx::query("DELETE FROM folder_queue WHERE folder_id = ? AND queue_id = ?")
                .bind(folder_id)
                .bind(queue_id)
                .execute(&mut *tx)
                .await?;
        } else {
            sqlx::query("INSERT INTO folder_queue (id, folder_id, queue_id, sort_order) VALUES (?, ?, ?, 0)").bind(Uuid::new_v4().to_string()).bind(folder_id).bind(queue_id).execute(&mut *tx).await?;
        }
        tx.commit().await?;
        Ok(!existing)
    }

    pub async fn reorder_queues(
        &self,
        folder_id: &str,
        ordered_queue_ids: &[String],
    ) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        for (index, queue_id) in ordered_queue_ids.iter().enumerate() {
            sqlx::query(
                "UPDATE folder_queue SET sort_order = ? WHERE folder_id = ? AND queue_id = ?",
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
        Ok(sqlx::query_as::<_, Queue>("SELECT q.id, q.connection_id, q.queue_name, q.display_name, q.is_starred, q.auto_refresh_rate, q.notification_settings, q.created_at FROM queue q JOIN folder_queue fq ON fq.queue_id = q.id WHERE fq.folder_id = ? ORDER BY fq.sort_order").bind(folder_id).fetch_all(&self.pool).await?)
    }
}
