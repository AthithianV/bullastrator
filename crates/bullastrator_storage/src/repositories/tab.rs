use anyhow::{Context, Result, bail};
use sqlx::SqlitePool;

use crate::models::{CreateTab, Tab, UpdateTab};

#[derive(Clone)]
pub struct TabRepository {
    pool: SqlitePool,
}

impl TabRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, id: &str, workspace_id: &str, data: CreateTab) -> Result<Tab> {
        if let Some(existing) = self.get_by_id(id).await? {
            self.set_active_tab(workspace_id, Some(existing.id.clone()))
                .await?;

            return Ok(existing);
        }

        let rank: i32 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(MAX(rank), -1) + 1
            FROM tabs
            WHERE workspace_id = ?
            "#,
        )
        .bind(workspace_id)
        .fetch_one(&self.pool)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO tabs (
                id,
                workspace_id,
                connection_id,
                user_id,
                title,
                params,
                is_preview,
                rank
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(workspace_id)
        .bind(data.connection_id)
        .bind(data.user_id)
        .bind(data.title)
        .bind(data.params)
        .bind(data.is_preview)
        .bind(rank)
        .execute(&self.pool)
        .await
        .context("Failed to create tab")?;

        let result = self
            .get_by_id(id)
            .await?
            .context("Created tab was not found")?;

        self.set_active_tab(workspace_id, Some(id.to_string()))
            .await?;

        Ok(result)
    }

    pub async fn get_all(&self, workspace_id: &str) -> Result<Vec<Tab>> {
        let rows = sqlx::query_as::<_, Tab>(
            r#"
                SELECT
                    t.id AS id,
                    t.workspace_id AS workspace_id,
                    t.connection_id AS connection_id,
                    t.user_id AS user_id,
                    t.title AS title,
                    t.params AS params,
                    t.is_active AS is_active,
                    t.is_dirty AS is_dirty,
                    t.is_pinned AS is_pinned,
                    t.is_preview AS is_preview,
                    t.rank AS rank,
                    c.color AS connection_color,
                    c.label AS connection_label
                FROM
                    tabs t
                LEFT JOIN connections c ON c.id = t.connection_id
                WHERE t.workspace_id = ?
                ORDER BY t.rank
            "#,
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().collect())
    }
    pub async fn get_by_id(&self, id: &str) -> Result<Option<Tab>> {
        let row = sqlx::query_as::<_, Tab>(
            r#"
                SELECT
                    t.id AS id,
                    t.workspace_id AS workspace_id,
                    t.connection_id AS connection_id,
                    t.user_id AS user_id,
                    t.title AS title,
                    t.params AS params,
                    t.is_active AS is_active,
                    t.is_dirty AS is_dirty,
                    t.is_pinned AS is_pinned,
                    t.is_preview AS is_preview,
                    t.rank AS rank,
                    c.color AS connection_color,
                    c.label AS connection_label
                FROM
                    tabs t
                LEFT JOIN
                    connections c
                ON
                    c.id = t.connection_id
                WHERE
                    t.id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn update(&self, id: &str, data: UpdateTab) -> Result<Tab> {
        let result = sqlx::query(
            r#"
            UPDATE tabs
            SET
                title = COALESCE(?, title),
                params = COALESCE(?, params),
                rank = COALESCE(?, rank),
                is_active = COALESCE(?, is_active),
                is_dirty = COALESCE(?, is_dirty),
                is_pinned = COALESCE(?, is_pinned),
                is_preview = COALESCE(?, is_preview),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(data.title)
        .bind(data.params)
        .bind(data.rank)
        .bind(data.is_active)
        .bind(data.is_dirty)
        .bind(data.is_pinned)
        .bind(data.is_preview)
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            bail!("Tab with ID {id} not found");
        }

        let tab = self
            .get_by_id(id)
            .await?
            .context("Updated tab was not found")?;

        self.set_active_tab(&tab.workspace_id, Some(id.to_string()))
            .await?;

        Ok(tab)
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM tabs
            WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            bail!("Tab with ID {id} not found");
        }

        Ok(result.rows_affected())
    }

    pub async fn update_order(&self, workspace_id: &str, ordered_ids: &[String]) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for (index, id) in ordered_ids.iter().enumerate() {
            sqlx::query(
                r#"
                UPDATE tabs
                SET rank = ?
                WHERE id = ?
                  AND workspace_id = ?
                "#,
            )
            .bind(index as i32)
            .bind(id)
            .bind(workspace_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await.context("Failed to commit tab reorder")
    }

    pub async fn set_active_tab(&self, workspace_id: &str, tab_id: Option<String>) -> Result<()> {
        let result = sqlx::query(
            r#"
            UPDATE workspaces
            SET active_tab_id = ?
            WHERE id = ?
            "#,
        )
        .bind(tab_id)
        .bind(workspace_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            bail!("Workspace with ID {workspace_id} not found");
        }

        Ok(())
    }

    pub async fn get_active_tab(&self, workspace_id: &str) -> Result<Option<String>> {
        Ok(sqlx::query_scalar(
            r#"
                SELECT active_tab_id
                FROM workspaces
                WHERE id = ?
                "#,
        )
        .bind(workspace_id)
        .fetch_optional(&self.pool)
        .await?
        .flatten())
    }
}
