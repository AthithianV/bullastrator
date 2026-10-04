use anyhow::{bail, Context, Result};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{
    CreateWorkspace, CreateWorkspaceMember, Settings, UpdateWorkspace, UpdateWorkspaceMember,
    Workspace, WorkspaceMember,
};

#[derive(Clone)]
pub struct WorkspaceRepository {
    pool: SqlitePool,
}

impl WorkspaceRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, data: CreateWorkspace) -> Result<Workspace> {
        let id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO workspace (id, user_id, name, color, icon, plan, role, max_connections, last_accessed_at, is_guest_mode, is_primary) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 0)")
            .bind(&id).bind(data.user_id).bind(data.name).bind(data.color).bind(data.icon).bind(data.plan).bind(data.role).bind(data.max_connections).bind(data.last_accessed_at).execute(&self.pool).await?;
        self.get_by_id(&id)
            .await?
            .context("Created workspace was not found")
    }

    pub async fn get_by_user_id(&self, user_id: &str) -> Result<Vec<Workspace>> {
        Ok(sqlx::query_as::<_, Workspace>("SELECT id, user_id, name, color, active_tab_id, icon, last_accessed_at, created_at, plan, role, max_connections, is_guest_mode, is_primary FROM workspace WHERE user_id = ? ORDER BY created_at").bind(user_id).fetch_all(&self.pool).await?)
    }

    pub async fn get_all(&self) -> Result<Vec<Workspace>> {
        Ok(sqlx::query_as::<_, Workspace>("SELECT id, user_id, name, color, active_tab_id, icon, last_accessed_at, created_at, plan, role, max_connections, is_guest_mode, is_primary FROM workspace WHERE is_guest_mode = 0 ORDER BY created_at").fetch_all(&self.pool).await?)
    }

    pub async fn get_active_workspace(&self) -> Result<Workspace> {
        let setting = sqlx::query_as::<_, Settings>(
            "SELECT key, user_id, value FROM settings WHERE key = 'active_workspace_id'",
        )
        .fetch_optional(&self.pool)
        .await?;
        if let Some(Some(id)) = setting.map(|s| s.value) {
            if let Some(workspace) = self.get_by_id(&id).await? {
                return Ok(workspace);
            }
        }
        let workspace = sqlx::query_as::<_, Workspace>("SELECT id, user_id, name, color, active_tab_id, icon, last_accessed_at, created_at, plan, role, max_connections, is_guest_mode, is_primary FROM workspace ORDER BY last_accessed_at DESC LIMIT 1").fetch_optional(&self.pool).await?.context("No workspaces found in the database")?;
        self.set_active_workspace(&workspace.id).await?;
        Ok(workspace)
    }

    pub async fn set_active_workspace(&self, workspace_id: &str) -> Result<()> {
        sqlx::query("INSERT INTO settings (key, user_id, value) VALUES ('active_workspace_id', NULL, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value, user_id = excluded.user_id").bind(workspace_id).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn go_guest_mode(&self) -> Result<Workspace> {
        let workspace = sqlx::query_as::<_, Workspace>("SELECT id, user_id, name, color, active_tab_id, icon, last_accessed_at, created_at, plan, role, max_connections, is_guest_mode, is_primary FROM workspace WHERE is_guest_mode = 1 LIMIT 1").fetch_optional(&self.pool).await?.context("Guest mode workspace not found")?;
        self.set_active_workspace(&workspace.id).await?;
        Ok(workspace)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Workspace>> {
        Ok(sqlx::query_as::<_, Workspace>("SELECT id, user_id, name, color, active_tab_id, icon, last_accessed_at, created_at, plan, role, max_connections, is_guest_mode, is_primary FROM workspace WHERE id = ?").bind(id).fetch_optional(&self.pool).await?)
    }

    pub async fn update(&self, id: &str, data: UpdateWorkspace) -> Result<Workspace> {
        let result = sqlx::query("UPDATE workspace SET name = COALESCE(?, name), icon = COALESCE(?, icon), color = COALESCE(?, color), last_accessed_at = COALESCE(?, last_accessed_at), is_guest_mode = COALESCE(?, is_guest_mode), is_primary = COALESCE(?, is_primary) WHERE id = ?")
            .bind(data.name).bind(data.icon).bind(data.color).bind(data.last_accessed_at).bind(data.is_guest_mode).bind(data.is_primary).bind(id).execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            bail!("Workspace ID {id} not found")
        }
        self.get_by_id(id)
            .await?
            .context("Updated workspace was not found")
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        let result = sqlx::query("DELETE FROM workspace WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        if result.rows_affected() == 0 {
            bail!("Workspace ID {id} not found")
        }
        Ok(result.rows_affected())
    }

    pub async fn upsert(&self, id: &str, data: CreateWorkspace) -> Result<Workspace> {
        sqlx::query("INSERT INTO workspace (id, user_id, name, color, icon, plan, role, max_connections, last_accessed_at, is_guest_mode, is_primary) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 0) ON CONFLICT(id) DO UPDATE SET user_id = excluded.user_id, name = excluded.name, plan = excluded.plan, role = excluded.role, max_connections = excluded.max_connections, icon = excluded.icon, color = excluded.color")
            .bind(id).bind(data.user_id).bind(data.name).bind(data.color).bind(data.icon).bind(data.plan).bind(data.role).bind(data.max_connections).bind(data.last_accessed_at).execute(&self.pool).await?;
        self.get_by_id(id)
            .await?
            .context("Upserted workspace was not found")
    }

    pub async fn select_workspace(&self, id: &str) -> Result<Workspace> {
        let workspace = self.get_by_id(id).await?.context("Workspace not found")?;
        sqlx::query("UPDATE workspace SET last_accessed_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        self.set_active_workspace(id).await?;
        Ok(workspace)
    }

    pub async fn get_members(&self, connection_id: &str) -> Result<Vec<WorkspaceMember>> {
        Ok(sqlx::query_as::<_, WorkspaceMember>(
            "SELECT user_id, connection_id, role, created_at FROM workspace_members WHERE connection_id = ? ORDER BY created_at",
        )
        .bind(connection_id)
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn get_member(&self, user_id: &str, connection_id: &str) -> Result<Option<WorkspaceMember>> {
        Ok(sqlx::query_as::<_, WorkspaceMember>(
            "SELECT user_id, connection_id, role, created_at FROM workspace_members WHERE user_id = ? AND connection_id = ?",
        )
        .bind(user_id)
        .bind(connection_id)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn add_member(&self, data: CreateWorkspaceMember) -> Result<WorkspaceMember> {
        sqlx::query(
            "INSERT INTO workspace_members (user_id, connection_id, role) VALUES (?, ?, ?)",
        )
        .bind(&data.user_id)
        .bind(&data.connection_id)
        .bind(&data.role)
        .execute(&self.pool)
        .await?;
        self.get_member(&data.user_id, &data.connection_id)
            .await?
            .context("Added workspace member was not found")
    }

    pub async fn update_member(
        &self,
        user_id: &str,
        connection_id: &str,
        data: UpdateWorkspaceMember,
    ) -> Result<WorkspaceMember> {
        let result = sqlx::query(
            "UPDATE workspace_members SET role = COALESCE(?, role) WHERE user_id = ? AND connection_id = ?",
        )
        .bind(data.role)
        .bind(user_id)
        .bind(connection_id)
        .execute(&self.pool)
        .await?;
        if result.rows_affected() == 0 {
            bail!("Workspace member was not found")
        }
        self.get_member(user_id, connection_id)
            .await?
            .context("Updated workspace member was not found")
    }

    pub async fn remove_member(&self, user_id: &str, connection_id: &str) -> Result<u64> {
        let result = sqlx::query(
            "DELETE FROM workspace_members WHERE user_id = ? AND connection_id = ?",
        )
        .bind(user_id)
        .bind(connection_id)
        .execute(&self.pool)
        .await?;
        if result.rows_affected() == 0 {
            bail!("Workspace member was not found")
        }
        Ok(result.rows_affected())
    }
}
