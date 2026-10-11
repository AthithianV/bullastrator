use std::collections::HashMap;
use std::sync::Arc;

use crate::error::{BullastratorError, Result};
use crate::services::{
    RedisConnection, folder::FolderService, queue::QueueService, tab::TabService,
    user::UserService, workspace::WorkspaceService,
};
use crate::utils::redis_connection::create_redis_connection;
use bullastrator_storage::models::CreateWorkspace;
use bullastrator_storage::repositories::{
    ConnectionRepository, FolderRepository, QueueRepository, TabRepository, UserRepository,
    WorkspaceRepository,
};
use sqlx::SqlitePool;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub users: UserService,
    pub connections: crate::services::connection::ConnectionService,
    pub redis_connections: Arc<RwLock<HashMap<String, RedisConnection>>>,
    pub queue_services: Arc<RwLock<HashMap<String, QueueService>>>,
    pub workspaces: WorkspaceService,
    pub tabs: TabService,
    pub folders: FolderService,
    pub settings: ServerSettings,
}

#[derive(Clone)]
pub struct ServerSettings {
    pub theme_color: String,
    pub vpn_restricted: bool,
}

impl AppState {
    pub async fn new(pool: SqlitePool) -> Result<Self> {
        Self::new_with_settings(
            pool,
            ServerSettings {
                theme_color: "#00CADB".into(),
                vpn_restricted: false,
            },
        )
        .await
    }

    pub async fn new_with_settings(pool: SqlitePool, settings: ServerSettings) -> Result<Self> {
        tracing::info!("initializing application state");
        let redis_connections = Arc::new(RwLock::new(HashMap::new()));
        let queue_services = Arc::new(RwLock::new(HashMap::new()));
        let workspaces = WorkspaceService::new(WorkspaceRepository::new(pool.clone()));
        let connections = crate::services::connection::ConnectionService::new(
            ConnectionRepository::new(pool.clone()),
            workspaces.clone(),
        );

        tracing::info!("application state initialized");

        Ok(Self {
            users: UserService::new(UserRepository::new(pool.clone())),
            connections: connections.clone(),
            workspaces: workspaces.clone(),
            tabs: TabService::new(TabRepository::new(pool.clone()), workspaces),
            folders: FolderService::new(FolderRepository::new(pool.clone()), connections),
            db: pool,
            redis_connections,
            queue_services,
            settings,
        })
    }

    pub async fn initialize_default_desktop_user(&self) -> Result<String> {
        let user = self.users.ensure_default_user().await?;

        if self.workspaces.active(&user.id).await.is_err() {
            let workspace = self
                .workspaces
                .create(
                    &user.id,
                    CreateWorkspace {
                        name: "Bullastrator".into(),
                        color: Some("#00CADB".into()),
                        icon: None,
                    },
                )
                .await?;
            self.workspaces.select(&workspace.id, &user.id).await?;
        }

        Ok(user.id)
    }

    pub async fn get_redis_connection(&self, connection_id: &str) -> Result<RedisConnection> {
        if let Some(redis_connection) = self
            .redis_connections
            .read()
            .await
            .get(connection_id)
            .cloned()
        {
            return Ok(redis_connection);
        }

        let connection = ConnectionRepository::new(self.db.clone())
            .get_credential_insecure(connection_id)
            .await?;

        if connection.is_none() {
            return Err(BullastratorError::NotFound(
                "connection not found".to_string(),
            ));
        }
        let connection = connection.unwrap();

        tracing::debug!(connection_id = %connection_id, host = %connection.host, "initializing Redis connection");
        let redis_connection = create_redis_connection(&connection)?;
        let connection_id = connection_id.to_string();

        let mut redis_connections = self.redis_connections.write().await;

        let connection = redis_connections
            .entry(connection_id.to_string())
            .or_insert(redis_connection);

        Ok(connection.clone())
    }

    pub async fn get_queue_service(&self, connection_id: &str) -> Result<QueueService> {
        if let Some(queue_service) = self.queue_services.read().await.get(connection_id).cloned() {
            return Ok(queue_service);
        }

        let queue_service =
            self.get_redis_connection(connection_id)
                .await
                .map(|redis_connection| {
                    QueueService::new(
                        redis_connection.clone(),
                        QueueRepository::new(self.db.clone()),
                        self.connections.clone(),
                    )
                })?;

        let mut queue_services = self.queue_services.write().await;

        let queue_service_clone = queue_services
            .entry(connection_id.to_string())
            .or_insert(queue_service);

        Ok(queue_service_clone.clone())
    }
}
