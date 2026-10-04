use anyhow::{Context, Result, bail};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{
    CreateWorkspace, CreateWorkspaceMember, UpdateWorkspace, UpdateWorkspaceMember, Workspace,
    WorkspaceMember, workspace::WorkspacePermissionError,
};

#[derive(Clone)]
pub struct WorkspaceRepository {
    pool: SqlitePool,
}

impl WorkspaceRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn role_level(role: &str) -> Result<u8, WorkspacePermissionError> {
        match role {
            "VIEWER" => Ok(1),
            "EDITOR" => Ok(2),
            "ADMIN" => Ok(3),
            "OWNER" => Ok(4),
            _ => Err(WorkspacePermissionError::InvalidRole(role.to_string())),
        }
    }

    pub async fn check_permission(
        &self,
        workspace_id: &str,
        user_id: &str,
        required_role: &str,
    ) -> Result<(), WorkspacePermissionError> {
        let actual_role: Option<String> = sqlx::query_scalar(
            r#"
                SELECT wm.role
                FROM workspace_members wm
                WHERE wm.workspace_id = ?
                  AND wm.user_id = ?
            "#,
        )
        .bind(workspace_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| WorkspacePermissionError::InvalidRole(e.to_string()))?;

        let Some(actual_role) = actual_role else {
            return Err(WorkspacePermissionError::NotMember);
        };

        let actual_level = Self::role_level(&actual_role)?;
        let required_level = Self::role_level(required_role)?;

        if actual_level < required_level {
            return Err(WorkspacePermissionError::InsufficientRole {
                required: required_role.to_string(),
                actual: actual_role,
            });
        }

        Ok(())
    }

    pub async fn create(&self, data: CreateWorkspace) -> Result<Workspace> {
        let exists: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM workspaces
                WHERE user_id = ?
                  AND LOWER(name) = LOWER(?)
            )
            "#,
        )
        .bind(&data.user_id)
        .bind(&data.name)
        .fetch_one(&self.pool)
        .await?;

        if exists {
            anyhow::bail!("A workspace with this name already exists");
        }

        let id = Uuid::new_v4().to_string();

        sqlx::query(
            r#"
                INSERT INTO
                    workspace (
                        id,
                        user_id,
                        name,
                        color,
                        icon,
                    ) VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(data.user_id.clone())
        .bind(data.name)
        .bind(data.color)
        .bind(data.icon)
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
                INSERT INTO
                    workspace_members (
                        workspace_id,
                        user_id,
                        role,
                        created_at
                    ) VALUES (?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(data.user_id.clone())
        .bind("OWNER")
        .execute(&self.pool)
        .await?;

        self.get_by_id(&id, &data.user_id)
            .await?
            .context("Created workspace was not found")
    }

    pub async fn get_by_user_id(&self, user_id: &str) -> Result<Vec<Workspace>> {
        Ok(sqlx::query_as::<_, Workspace>(
            r#"
                SELECT
                    w.id,
                    w.user_id,
                    w.name,
                    w.color,
                    w.active_tab_id,
                    w.icon,
                    w.last_accessed_at,
                    ws.role
                FROM
                    workspace w
                INNER JOIN
                    workspace_members wm
                ON
                    wm.workspace_id = w.id
                WHERE
                    wm.user_id = ?
                ORDER BY last_accessed_at
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn get_active_workspace(&self, user_id: &str) -> Result<Workspace> {
        // 1. Get the user's currently selected workspace.
        let active_workspace_id: Option<String> = sqlx::query_scalar(
            r#"
            SELECT active_workspace_id
            FROM users
            WHERE id = ?
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .flatten();

        // 2. If it exists and the user still has access to it, return it.
        if let Some(workspace_id) = active_workspace_id {
            if let Some(workspace) = sqlx::query_as::<_, Workspace>(
                r#"
                SELECT
                    w.id,
                    w.name,
                    w.color,
                    w.active_tab_id,
                    w.icon,
                    w.last_accessed_at,
                    w.is_primary,
                    wm.role
                FROM workspaces w
                INNER JOIN workspace_members wm
                    ON wm.workspace_id = w.id
                WHERE w.id = ?
                  AND wm.user_id = ?
                LIMIT 1
                "#,
            )
            .bind(&workspace_id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?
            {
                return Ok(workspace);
            }
        }

        // 3. Active workspace doesn't exist or user no longer has access.
        //    Find the most recently accessed workspace.
        let workspace = sqlx::query_as::<_, Workspace>(
            r#"
            SELECT
                w.id,
                w.name,
                w.color,
                w.active_tab_id,
                w.icon,
                w.last_accessed_at,
                w.created_at,
                w.is_primary,
                wm.role
            FROM workspaces w
            INNER JOIN workspace_members wm
                ON wm.workspace_id = w.id
            WHERE wm.user_id = ?
            ORDER BY w.last_accessed_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        // 4. No workspace at all.
        let Some(workspace) = workspace else {
            anyhow::bail!("User does not belong to any workspace");
        };

        // 5. Save the fallback workspace as the user's active workspace.
        sqlx::query(
            r#"
            UPDATE users
            SET active_workspace_id = ?,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(&workspace.id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(workspace)
    }

    pub async fn set_active_workspace(
        &self,
        workspace_id: &str,
        user_id: &str,
    ) -> Result<Workspace> {
        // 5. Save the fallback workspace as the user's active workspace.
        sqlx::query(
            r#"
            UPDATE users
            SET active_workspace_id = ?,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(workspace_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            UPDATE workspaces
            SET last_accessed_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(workspace_id)
        .execute(&self.pool)
        .await?;

        self.get_by_id(workspace_id, user_id)
            .await?
            .context("Created workspace was not found")
    }

    pub async fn get_by_id(&self, workspace_id: &str, user_id: &str) -> Result<Option<Workspace>> {
        Ok(sqlx::query_as::<_, Workspace>(
            r#"
                SELECT
                    w.id,
                    w.name,
                    w.color,
                    w.active_tab_id,
                    w.icon,
                    w.last_accessed_at,
                    w.created_at,
                    w.is_primary,
                    wm.role
                FROM workspaces w
                INNER JOIN workspace_members wm
                    ON wm.workspace_id = w.id
                WHERE w.id = ? AND wm.user_id = ?
                ORDER BY w.last_accessed_at DESC
                LIMIT 1
            "#,
        )
        .bind(workspace_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn update(
        &self,
        workspace_id: &str,
        user_id: &str,
        data: UpdateWorkspace,
    ) -> Result<Workspace> {
        let result = sqlx::query(
            r#"
                UPDATE
                    workspace
                SET
                    name = COALESCE(?, name),
                    icon = COALESCE(?, icon),
                    color = COALESCE(?, color)
                WHERE id = ?
            "#,
        )
        .bind(data.name)
        .bind(data.icon)
        .bind(data.color)
        .bind(workspace_id)
        .execute(&self.pool)
        .await?;
        if result.rows_affected() == 0 {
            bail!("Workspace ID {workspace_id} not found")
        }

        self.get_by_id(workspace_id, user_id)
            .await?
            .context("Created workspace was not found")
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

    pub async fn get_members(&self, workspace_id: &str) -> Result<Vec<WorkspaceMember>> {
        Ok(sqlx::query_as::<_, WorkspaceMember>(
            r#"
                SELECT
                    wm.user_id,
                    wm.workspace_id,
                    wm.role,
                    u.name,
                    u.email
                FROM workspace_members wm
                INNER JOIN users u
                    ON u.id = wm.user_id
                WHERE wm.workspace_id = ?
                ORDER BY wm.created_at
            "#,
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await?)
    }

    pub async fn get_member(
        &self,
        user_id: &str,
        connection_id: &str,
    ) -> Result<Option<WorkspaceMember>> {
        Ok(sqlx::query_as::<_, WorkspaceMember>(
            "SELECT user_id, connection_id, role, created_at FROM workspace_members WHERE user_id = ? AND connection_id = ?",
        )
        .bind(user_id)
        .bind(connection_id)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn add_member(&self, data: CreateWorkspaceMember) -> Result<WorkspaceMember> {
        sqlx::query("INSERT INTO workspace_members (user_id, workspace_id, role) VALUES (?, ?, ?)")
            .bind(&data.user_id)
            .bind(&data.workspace_id)
            .bind(&data.role)
            .execute(&self.pool)
            .await?;
        self.get_member(&data.user_id, &data.workspace_id)
            .await?
            .context("Added workspace member was not found")
    }

    pub async fn update_member(
        &self,
        user_id: &str,
        workspace_id: &str,
        data: UpdateWorkspaceMember,
    ) -> Result<WorkspaceMember> {
        let result = sqlx::query(
            "UPDATE workspace_members SET role = COALESCE(?, role) WHERE user_id = ? AND workspace_id = ?",
        )
        .bind(data.role)
        .bind(user_id)
        .bind(workspace_id)
        .execute(&self.pool)
        .await?;
        if result.rows_affected() == 0 {
            bail!("Workspace member was not found")
        }
        self.get_member(user_id, workspace_id)
            .await?
            .context("Updated workspace member was not found")
    }

    pub async fn remove_member(&self, user_id: &str, workspace_id: &str) -> Result<u64> {
        let result =
            sqlx::query("DELETE FROM workspace_members WHERE user_id = ? AND workspace_id = ?")
                .bind(user_id)
                .bind(workspace_id)
                .execute(&self.pool)
                .await?;
        if result.rows_affected() == 0 {
            bail!("Workspace member was not found")
        }
        Ok(result.rows_affected())
    }
}
