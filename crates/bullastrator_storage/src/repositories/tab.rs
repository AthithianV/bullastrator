use anyhow::{bail, Context, Result};
use sqlx::SqlitePool;

use crate::models::{Connection, CreateTab, Tab, TabWithConnection, UpdateTab};

pub struct TabRepository {
    pool: SqlitePool,
}

impl TabRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        id: &str,
        workspace_id: &str,
        data: CreateTab,
    ) -> Result<TabWithConnection> {
        if let Some(existing) = self.get_by_id(id).await? {
            self.set_active_tab(workspace_id, Some(existing.tab.id.clone()))
                .await?;
            return Ok(existing);
        }
        let rank: i32 = sqlx::query_scalar(
            "SELECT COALESCE(MAX(rank), -1) + 1 FROM tab WHERE workspace_id = ?",
        )
        .bind(workspace_id)
        .fetch_one(&self.pool)
        .await?;
        sqlx::query("INSERT INTO tab (id, workspace_id, connection_id, user_id, title, params, is_preview, rank) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(id).bind(workspace_id).bind(data.connection_id).bind(data.user_id).bind(data.title).bind(data.params).bind(data.is_preview).bind(rank).execute(&self.pool).await.context("Failed to create tab")?;
        let result = self
            .get_by_id(id)
            .await?
            .context("Created tab was not found")?;
        self.set_active_tab(workspace_id, Some(id.to_string()))
            .await?;
        Ok(result)
    }

    pub async fn get_all(&self, workspace_id: &str) -> Result<Vec<TabWithConnection>> {
        let tabs = sqlx::query_as::<_, Tab>("SELECT id, workspace_id, connection_id, user_id, title, params, is_active, is_dirty, is_pinned, is_preview, rank, created_at, updated_at FROM tab WHERE workspace_id = ? ORDER BY rank").bind(workspace_id).fetch_all(&self.pool).await?;
        self.attach_connections(tabs).await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<TabWithConnection>> {
        let tab = sqlx::query_as::<_, Tab>("SELECT id, workspace_id, connection_id, user_id, title, params, is_active, is_dirty, is_pinned, is_preview, rank, created_at, updated_at FROM tab WHERE id = ?").bind(id).fetch_optional(&self.pool).await?;
        match tab {
            Some(tab) => Ok(Some(self.attach_connections(vec![tab]).await?.remove(0))),
            None => Ok(None),
        }
    }

    async fn attach_connections(&self, tabs: Vec<Tab>) -> Result<Vec<TabWithConnection>> {
        let mut result = Vec::with_capacity(tabs.len());
        for tab in tabs {
            let connection = match &tab.connection_id { Some(id) => sqlx::query_as::<_, Connection>("SELECT id, workspace_id, name, host, port, password, username, db, last_synced_at, bullmq_prefix, is_tls_enabled, color, label, created_at FROM connection WHERE id = ?").bind(id).fetch_optional(&self.pool).await?, None => None };
            result.push(TabWithConnection { tab, connection });
        }
        Ok(result)
    }

    pub async fn update(&self, id: &str, data: UpdateTab) -> Result<TabWithConnection> {
        let result = sqlx::query("UPDATE tab SET title = COALESCE(?, title), params = COALESCE(?, params), rank = COALESCE(?, rank), is_active = COALESCE(?, is_active), is_dirty = COALESCE(?, is_dirty), is_pinned = COALESCE(?, is_pinned), is_preview = COALESCE(?, is_preview), updated_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(data.title).bind(data.params).bind(data.rank).bind(data.is_active).bind(data.is_dirty).bind(data.is_pinned).bind(data.is_preview).bind(id).execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            bail!("Tab with ID {id} not found")
        }
        let tab = self
            .get_by_id(id)
            .await?
            .context("Updated tab was not found")?;
        self.set_active_tab(&tab.tab.workspace_id, Some(id.to_string()))
            .await?;
        Ok(tab)
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        let result = sqlx::query("DELETE FROM tab WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        if result.rows_affected() == 0 {
            bail!("Tab with ID {id} not found")
        }
        Ok(result.rows_affected())
    }

    pub async fn update_order(&self, workspace_id: &str, ordered_ids: &[String]) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        for (index, id) in ordered_ids.iter().enumerate() {
            sqlx::query("UPDATE tab SET rank = ? WHERE id = ? AND workspace_id = ?")
                .bind(index as i32)
                .bind(id)
                .bind(workspace_id)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await.context("Failed to commit tab reorder")
    }

    pub async fn set_active_tab(&self, workspace_id: &str, tab_id: Option<String>) -> Result<()> {
        let result = sqlx::query("UPDATE workspace SET active_tab_id = ? WHERE id = ?")
            .bind(tab_id)
            .bind(workspace_id)
            .execute(&self.pool)
            .await?;
        if result.rows_affected() == 0 {
            bail!("Workspace with ID {workspace_id} not found")
        }
        Ok(())
    }

    pub async fn get_active_tab(&self, workspace_id: &str) -> Result<Option<String>> {
        Ok(
            sqlx::query_scalar("SELECT active_tab_id FROM workspace WHERE id = ?")
                .bind(workspace_id)
                .fetch_optional(&self.pool)
                .await?
                .flatten(),
        )
    }
}
