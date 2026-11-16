use anyhow::{Result, bail};
use sqlx::SqlitePool;

use crate::models::{CreateUserRole, Paginated, RoleEntityOption, UpdateUserRole, UserRole};

#[derive(Clone)]
pub struct UserRoleRepository {
    pool: SqlitePool,
}

impl UserRoleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

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

    pub async fn list(
        &self,
        user_id: Option<&str>,
        entity_type: Option<&str>,
        search: Option<&str>,
        offset: usize,
        limit: usize,
    ) -> Result<Paginated<UserRole>> {
        let limit = limit.clamp(1, 100);
        let fetch_limit = limit + 1;
        let entity_type = entity_type.unwrap_or("ALL");
        let query = match entity_type {
            "WORKSPACE" => {
                r#"
                SELECT
                    'WORKSPACE:' || wm.user_id || ':' || wm.workspace_id AS id,
                    wm.user_id,
                    'WORKSPACE' AS entity_type,
                    wm.workspace_id AS entity_id,
                    w.name AS entity_name,
                    wm.role,
                    wm.created_at,
                    NULL AS updated_at
                FROM workspace_members wm
                JOIN workspaces w ON w.id = wm.workspace_id
                WHERE wm.user_id = ?
                  AND (? IS NULL OR lower(w.name) LIKE '%' || lower(?) || '%')
                ORDER BY entity_name
                LIMIT ? OFFSET ?
                "#
            }
            "CONNECTION" => {
                r#"
                SELECT
                    'CONNECTION:' || r.user_id || ':' || r.connection_id AS id,
                    r.user_id,
                    'CONNECTION' AS entity_type,
                    r.connection_id AS entity_id,
                    c.name AS entity_name,
                    r.role,
                    NULL AS created_at,
                    NULL AS updated_at
                FROM user_connection_roles r
                JOIN connections c ON c.id = r.connection_id
                WHERE r.user_id = ?
                  AND (? IS NULL OR lower(c.name) LIKE '%' || lower(?) || '%')
                ORDER BY entity_name
                LIMIT ? OFFSET ?
                "#
            }
            "FOLDER" => {
                r#"
                SELECT
                    'FOLDER:' || r.user_id || ':' || r.folder_id AS id,
                    r.user_id,
                    'FOLDER' AS entity_type,
                    r.folder_id AS entity_id,
                    f.title AS entity_name,
                    r.role,
                    NULL AS created_at,
                    NULL AS updated_at
                FROM user_folder_roles r
                JOIN folders f ON f.id = r.folder_id
                WHERE r.user_id = ?
                  AND (? IS NULL OR lower(f.title) LIKE '%' || lower(?) || '%')
                ORDER BY entity_name
                LIMIT ? OFFSET ?
                "#
            }
            "QUEUE" => {
                r#"
                SELECT
                    'QUEUE:' || r.user_id || ':' || r.queue_id AS id,
                    r.user_id,
                    'QUEUE' AS entity_type,
                    r.queue_id AS entity_id,
                    q.queue_name AS entity_name,
                    r.role,
                    NULL AS created_at,
                    NULL AS updated_at
                FROM user_queue_roles r
                JOIN queues q ON q.id = r.queue_id
                WHERE r.user_id = ?
                  AND (? IS NULL OR lower(q.queue_name) LIKE '%' || lower(?) || '%')
                ORDER BY entity_name
                LIMIT ? OFFSET ?
                "#
            }
            "ALL" => {
                r#"
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
                  AND (? IS NULL OR lower(entity_name) LIKE '%' || lower(?) || '%')
                ORDER BY entity_type, entity_name
                LIMIT ? OFFSET ?
                "#
            }
            _ => bail!("Unsupported role entity type: {entity_type}"),
        };

        let mut request = sqlx::query_as::<_, UserRole>(query);
        if entity_type == "ALL" {
            request = request
                .bind(user_id)
                .bind(user_id)
                .bind(search)
                .bind(search)
                .bind(fetch_limit as i64)
                .bind(offset as i64);
        } else {
            request = request
                .bind(user_id.unwrap_or_default())
                .bind(search)
                .bind(search)
                .bind(fetch_limit as i64)
                .bind(offset as i64);
        }
        let mut items = request.fetch_all(&self.pool).await?;
        let has_more = items.len() > limit;
        if has_more {
            items.pop();
        }
        Ok(Paginated {
            next_offset: has_more.then_some(offset + limit),
            has_more,
            items,
        })
    }

    pub async fn create(&self, data: CreateUserRole) -> Result<UserRole> {
        Self::validate_type(&data.entity_type)?;
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE id = ?)")
            .bind(&data.user_id)
            .fetch_one(&self.pool)
            .await?;
        if !exists {
            bail!("User was not found");
        }

        match data.entity_type.as_str() {
            "WORKSPACE" =>
                sqlx::
                    query(
                        "INSERT OR REPLACE INTO workspace_members (workspace_id, user_id, role) VALUES (?, ?, ?)"
                    )
                .bind(&data.entity_id)
                .bind(&data.user_id)
                .bind(&data.role)
                .execute(&self.pool).await?,
            "CONNECTION" => sqlx::query(
                "INSERT OR REPLACE INTO user_connection_roles (user_id, connection_id, role) VALUES (?, ?, ?)")
                .bind(&data.user_id)
                .bind(&data.entity_id)
                .bind(&data.role)
                .execute(&self.pool).await?,
            "FOLDER" => sqlx::query(
                "INSERT OR REPLACE INTO user_folder_roles (user_id, folder_id, role) VALUES (?, ?, ?)"
            )
                .bind(&data.user_id)
                .bind(&data.entity_id)
                .bind(&data.role)
                .execute(&self.pool).await?,
            "QUEUE" => sqlx::query("INSERT OR REPLACE INTO user_queue_roles (user_id, queue_id, role) VALUES (?, ?, ?)")
                .bind(&data.user_id)
                .bind(&data.entity_id)
                .bind(&data.role)
                .execute(&self.pool).await?,
            _ => unreachable!(),
        };
        self.get(&format!(
            "{}:{}:{}",
            data.entity_type, data.user_id, data.entity_id
        ))
        .await
    }

    pub async fn get(&self, id: &str) -> Result<UserRole> {
        let (entity_type, user_id, _) = Self::parse_id(id)?;
        self.list(Some(user_id), Some(entity_type), None, 0, 1)
            .await?
            .items
            .into_iter()
            .find(|role| role.id == id)
            .ok_or_else(|| anyhow::anyhow!("Role was not found"))
    }

    pub async fn update(&self, id: &str, data: UpdateUserRole) -> Result<UserRole> {
        let (entity_type, user_id, entity_id) = Self::parse_id(id)?;
        self.create(CreateUserRole {
            user_id: user_id.into(),
            entity_type: entity_type.into(),
            entity_id: entity_id.into(),
            role: data.role,
        })
        .await
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        let (entity_type, user_id, entity_id) = Self::parse_id(id)?;
        let result = match entity_type {
            "WORKSPACE" => {
                sqlx::query("DELETE FROM workspace_members WHERE user_id = ? AND workspace_id = ?")
                    .bind(user_id)
                    .bind(entity_id)
                    .execute(&self.pool)
                    .await?
            }
            "CONNECTION" => {
                sqlx::query(
                    "DELETE FROM user_connection_roles WHERE user_id = ? AND connection_id = ?",
                )
                .bind(user_id)
                .bind(entity_id)
                .execute(&self.pool)
                .await?
            }
            "FOLDER" => {
                sqlx::query("DELETE FROM user_folder_roles WHERE user_id = ? AND folder_id = ?")
                    .bind(user_id)
                    .bind(entity_id)
                    .execute(&self.pool)
                    .await?
            }
            "QUEUE" => {
                sqlx::query("DELETE FROM user_queue_roles WHERE user_id = ? AND queue_id = ?")
                    .bind(user_id)
                    .bind(entity_id)
                    .execute(&self.pool)
                    .await?
            }
            _ => unreachable!(),
        };
        Ok(result.rows_affected())
    }

    pub async fn options(
        &self,
        entity_type: &str,
        user_id: &str,
        search: Option<&str>,
        offset: usize,
        limit: usize,
    ) -> Result<Paginated<RoleEntityOption>> {
        let limit = limit.clamp(1, 100);
        let fetch_limit = limit + 1;
        Self::validate_type(entity_type)?;
        let query = match entity_type {
            "WORKSPACE" => {
                r#"
                SELECT
                    id,
                    name,
                    NULL AS workspace_id,
                    NULL AS workspace_name,
                    NULL AS connection_id,
                    NULL AS connection_name
                FROM workspaces
                WHERE (? IS NULL OR lower(name) LIKE '%' || lower(?) || '%')
                ORDER BY name COLLATE NOCASE
                LIMIT ? OFFSET ?
                "#
            }
            "CONNECTION" => {
                r#"
                SELECT
                    c.id,
                    c.name,
                    c.workspace_id,
                    w.name AS workspace_name,
                    NULL AS connection_id,
                    NULL AS connection_name
                FROM connections c
                JOIN workspaces w ON w.id = c.workspace_id
                WHERE (? IS NULL OR lower(c.name) LIKE '%' || lower(?) || '%' OR lower(w.name) LIKE '%' || lower(?) || '%')
                  AND EXISTS (
                    SELECT 1
                    FROM workspace_members wm
                    WHERE wm.workspace_id = c.workspace_id
                      AND wm.user_id = ?
                )
                ORDER BY c.name COLLATE NOCASE
                LIMIT ? OFFSET ?
                "#
            }
            "FOLDER" => {
                r#"
                SELECT
                    f.id,
                    f.title AS name,
                    c.workspace_id,
                    w.name AS workspace_name,
                    c.id AS connection_id,
                    c.name AS connection_name
                FROM folders f
                JOIN connections c ON c.id = f.connection_id
                JOIN workspaces w ON w.id = c.workspace_id
                WHERE (? IS NULL OR lower(f.title) LIKE '%' || lower(?) || '%' OR lower(c.name) LIKE '%' || lower(?) || '%' OR lower(w.name) LIKE '%' || lower(?) || '%')
                  AND EXISTS (
                    SELECT 1
                    FROM workspace_members wm
                    WHERE wm.workspace_id = c.workspace_id
                      AND wm.user_id = ?
                )
                  AND EXISTS (
                    SELECT 1
                    FROM user_connection_roles ucr
                    WHERE ucr.connection_id = c.id
                      AND ucr.user_id = ?
                )
                ORDER BY f.title COLLATE NOCASE
                LIMIT ? OFFSET ?
                "#
            }
            "QUEUE" => {
                r#"
                SELECT
                    q.id,
                    q.queue_name AS name,
                    c.workspace_id,
                    w.name AS workspace_name,
                    c.id AS connection_id,
                    c.name AS connection_name
                FROM queues q
                JOIN connections c ON c.id = q.connection_id
                JOIN workspaces w ON w.id = c.workspace_id
                WHERE (? IS NULL OR lower(q.queue_name) LIKE '%' || lower(?) || '%' OR lower(c.name) LIKE '%' || lower(?) || '%' OR lower(w.name) LIKE '%' || lower(?) || '%')
                  AND EXISTS (
                    SELECT 1
                    FROM workspace_members wm
                    WHERE wm.workspace_id = c.workspace_id
                      AND wm.user_id = ?
                )
                  AND EXISTS (
                    SELECT 1
                    FROM user_connection_roles ucr
                    WHERE ucr.connection_id = c.id
                      AND ucr.user_id = ?
                )
                ORDER BY q.queue_name COLLATE NOCASE
                LIMIT ? OFFSET ?
                "#
            }
            _ => unreachable!(),
        };
        let mut request = sqlx::query_as::<_, RoleEntityOption>(query);
        match entity_type {
            "CONNECTION" => {
                request = request.bind(search).bind(search).bind(search).bind(user_id);
            }
            "FOLDER" | "QUEUE" => {
                request = request
                    .bind(search)
                    .bind(search)
                    .bind(search)
                    .bind(search)
                    .bind(user_id)
                    .bind(user_id);
            }
            "WORKSPACE" => {
                request = request.bind(search).bind(search);
            }
            _ => unreachable!(),
        }
        let request = request
            .bind(fetch_limit as i64)
            .bind(offset as i64);
        let mut items = request.fetch_all(&self.pool).await?;
        let has_more = items.len() > limit;
        if has_more {
            items.pop();
        }
        Ok(Paginated {
            next_offset: has_more.then_some(offset + limit),
            has_more,
            items,
        })
    }
}
