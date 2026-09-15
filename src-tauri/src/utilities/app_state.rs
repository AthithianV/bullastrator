use anyhow::{Context, Result};
use deadpool_redis::Pool;
use sea_orm::DbConn;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::model::{connection_model::ReadConnectionModel, workspace_model::ReadWorkspaceModel};

#[derive(Clone)]
pub struct ConnectionPool {
    pub metadata: ReadConnectionModel,
    pub pool: Pool,
}

#[derive(Clone)]
pub struct AppState {
    pub app_db: Arc<RwLock<Option<DbConn>>>,
    pub active_workspace: Arc<RwLock<Option<ReadWorkspaceModel>>>,
    pub connection_pools: Arc<RwLock<HashMap<Uuid, ConnectionPool>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            app_db: Arc::new(RwLock::new(None)),
            active_workspace: Arc::new(RwLock::new(None)),
            connection_pools: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn set_active_workspace(&self, workspace: ReadWorkspaceModel) {
        let mut existing_workspace = self.active_workspace.write().await;
        *existing_workspace = Some(workspace);
    }

    pub async fn get_active_workspace(&self) -> Result<ReadWorkspaceModel, String> {
        let active_workspace_guard = self.active_workspace.read().await;
        active_workspace_guard
            .clone()
            .ok_or_else(|| "No Active Workspace".to_string())
    }

    pub async fn get_app_db_connection(&self) -> Result<DbConn, String> {
        let read_guard = self.app_db.read().await;
        read_guard
            .clone()
            .ok_or_else(|| "App DB not initialized yet".to_string())
    }

    pub async fn set_app_db_connection(&self, db: DbConn) -> () {
        let mut write_guard = self.app_db.write().await;
        *write_guard = Some(db);
    }

    pub async fn add_redis_connection(&self, id: Uuid, conn: ConnectionPool) {
        let mut connection_pools = self.connection_pools.write().await;
        connection_pools.insert(id, conn);
    }

    /// Retrieves a specific Redis connection by ID
    pub async fn get_redis_connection(&self, id: &Uuid) -> Result<ConnectionPool> {
        let connection_pools = self.connection_pools.read().await;

        connection_pools
            .get(id)
            .cloned() // MultiplexedConnection is cheap to clone
            .context(format!("Redis connection for ID '{}' not found", id))
    }

    /// Removes a connection if a workspace is closed
    pub async fn remove_redis_connection(&self, id: &Uuid) {
        let mut connection_pools = self.connection_pools.write().await;
        connection_pools.remove(id);
    }

    /// Update Connection
    pub async fn update_redis_connection(&self, id: Uuid, conn: ConnectionPool) {
        let mut connection_pools = self.connection_pools.write().await;
        connection_pools.insert(id, conn);
    }
}
