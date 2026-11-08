use anyhow::{Result, bail};
use sqlx::SqlitePool;

use crate::models::{CreateUserRole, RoleEntityOption, UpdateUserRole, UserRole};

#[derive(Clone)]
pub struct UserRoleRepository {
    pool: SqlitePool,
}

impl UserRoleRepository {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    fn validate_type(entity_type: &str) -> Result<()> {
        if matches!(entity_type, "WORKSPACE" | "CONNECTION" | "FOLDER" | "QUEUE") {
            Ok(())
        } else {
            bail!("Unsupported role entity type: {entity_type}")
        }
    }

    fn parse_id(id: &str) -> Result<(&str, &str, &str)> {
        let mut parts = id.splitn(3, ':');
        let entity_type = parts.next().unwrap_or_default();
        let user_id = parts.next().unwrap_or_default();
        let entity_id = parts.next().unwrap_or_default();
        Self::validate_type(entity_type)?;
        if user_id.is_empty() || entity_id.is_empty() {
            bail!("Invalid role id")
        }
        Ok((entity_type, user_id, entity_id))
    }

    pub async fn list(&self, user_id: Option<&str>) -> Result<Vec<UserRole>> {
        let query = r#"
            SELECT * FROM (
                SELECT 'WORKSPACE:' || wm.user_id || ':' || wm.workspace_id AS id,
                    wm.user_id, 'WORKSPACE' AS entity_type, wm.workspace_id AS entity_id,
                    w.name AS entity_name, wm.role, wm.created_at, NULL AS updated_at
                FROM workspace_members wm JOIN workspaces w ON w.id = wm.workspace_id
                UNION ALL
                SELECT 'CONNECTION:' || r.user_id || ':' || r.connection_id,
                    r.user_id, 'CONNECTION', r.connection_id, c.name, r.role, NULL, NULL
                FROM user_connection_roles r JOIN connections c ON c.id = r.connection_id
                UNION ALL
                SELECT 'FOLDER:' || r.user_id || ':' || r.folder_id,
                    r.user_id, 'FOLDER', r.folder_id, f.title, r.role, NULL, NULL
                FROM user_folder_roles r JOIN folders f ON f.id = r.folder_id
                UNION ALL
                SELECT 'QUEUE:' || r.user_id || ':' || r.queue_id,
                    r.user_id, 'QUEUE', r.queue_id, q.queue_name, r.role, NULL, NULL
                FROM user_queue_roles r JOIN queues q ON q.id = r.queue_id
            ) roles
            WHERE (? IS NULL OR user_id = ?)
            ORDER BY entity_type, entity_name
        "#;
        Ok(sqlx::query_as::<_, UserRole>(query)
            .bind(user_id).bind(user_id).fetch_all(&self.pool).await?)
    }

    pub async fn create(&self, data: CreateUserRole) -> Result<UserRole> {
        Self::validate_type(&data.entity_type)?;
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE id = ?)")
            .bind(&data.user_id).fetch_one(&self.pool).await?;
        if !exists { bail!("User was not found"); }

        match data.entity_type.as_str() {
            "WORKSPACE" => sqlx::query("INSERT OR REPLACE INTO workspace_members (workspace_id, user_id, role) VALUES (?, ?, ?)")
                .bind(&data.entity_id).bind(&data.user_id).bind(&data.role).execute(&self.pool).await?,
            "CONNECTION" => sqlx::query("INSERT OR REPLACE INTO user_connection_roles (user_id, connection_id, role) VALUES (?, ?, ?)")
                .bind(&data.user_id).bind(&data.entity_id).bind(&data.role).execute(&self.pool).await?,
            "FOLDER" => sqlx::query("INSERT OR REPLACE INTO user_folder_roles (user_id, folder_id, role) VALUES (?, ?, ?)")
                .bind(&data.user_id).bind(&data.entity_id).bind(&data.role).execute(&self.pool).await?,
            "QUEUE" => sqlx::query("INSERT OR REPLACE INTO user_queue_roles (user_id, queue_id, role) VALUES (?, ?, ?)")
                .bind(&data.user_id).bind(&data.entity_id).bind(&data.role).execute(&self.pool).await?,
            _ => unreachable!(),
        };
        self.get(&format!("{}:{}:{}", data.entity_type, data.user_id, data.entity_id)).await
    }

    pub async fn get(&self, id: &str) -> Result<UserRole> {
        self.list(None).await?.into_iter().find(|role| role.id == id).ok_or_else(|| anyhow::anyhow!("Role was not found"))
    }

    pub async fn update(&self, id: &str, data: UpdateUserRole) -> Result<UserRole> {
        let (entity_type, user_id, entity_id) = Self::parse_id(id)?;
        self.create(CreateUserRole { user_id: user_id.into(), entity_type: entity_type.into(), entity_id: entity_id.into(), role: data.role }).await
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        let (entity_type, user_id, entity_id) = Self::parse_id(id)?;
        let result = match entity_type {
            "WORKSPACE" => sqlx::query("DELETE FROM workspace_members WHERE user_id = ? AND workspace_id = ?").bind(user_id).bind(entity_id).execute(&self.pool).await?,
            "CONNECTION" => sqlx::query("DELETE FROM user_connection_roles WHERE user_id = ? AND connection_id = ?").bind(user_id).bind(entity_id).execute(&self.pool).await?,
            "FOLDER" => sqlx::query("DELETE FROM user_folder_roles WHERE user_id = ? AND folder_id = ?").bind(user_id).bind(entity_id).execute(&self.pool).await?,
            "QUEUE" => sqlx::query("DELETE FROM user_queue_roles WHERE user_id = ? AND queue_id = ?").bind(user_id).bind(entity_id).execute(&self.pool).await?,
            _ => unreachable!(),
        };
        Ok(result.rows_affected())
    }

    pub async fn options(&self, entity_type: &str) -> Result<Vec<RoleEntityOption>> {
        Self::validate_type(entity_type)?;
        let query = match entity_type {
            "WORKSPACE" => "SELECT id, name FROM workspaces ORDER BY name COLLATE NOCASE",
            "CONNECTION" => "SELECT id, name FROM connections ORDER BY name COLLATE NOCASE",
            "FOLDER" => "SELECT id, title AS name FROM folders ORDER BY name COLLATE NOCASE",
            "QUEUE" => "SELECT id, queue_name AS name FROM queues ORDER BY name COLLATE NOCASE",
            _ => unreachable!(),
        };
        Ok(sqlx::query_as::<_, RoleEntityOption>(query).fetch_all(&self.pool).await?)
    }
}
