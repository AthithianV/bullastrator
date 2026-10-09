use anyhow::{Context, Result, bail};
use semver::Version;
use std::time::Duration;
use tokio::time::timeout;

use bullastrator_storage::{
    models::{Connection, CreateConnection, UpdateConnection, WorkspaceRole},
    repositories::ConnectionRepository,
};

use crate::services::workspace::WorkspaceService;

#[derive(Clone)]
pub struct ConnectionService {
    repository: ConnectionRepository,
    workspaces: WorkspaceService,
}

impl ConnectionService {
    pub fn new(repository: ConnectionRepository, workspaces: WorkspaceService) -> Self {
        Self {
            repository,
            workspaces,
        }
    }

    async fn active_workspace(&self, user_id: &str) -> Result<String> {
        self.workspaces.active_id(user_id).await
    }

    pub async fn authorize(
        &self,
        connection_id: &str,
        user_id: &str,
        role: WorkspaceRole,
    ) -> Result<Connection> {
        let workspace_id = self.active_workspace(user_id).await?;
        let connection = self
            .repository
            .get_by_id(connection_id)
            .await?
            .context("Connection not found")?;
        if connection.workspace_id != workspace_id {
            bail!("Connection does not belong to the active workspace");
        }
        self.workspaces
            .check_permission(&workspace_id, user_id, role)
            .await?;
        Ok(connection)
    }

    pub async fn create(&self, user_id: &str, request: CreateConnection) -> Result<Connection> {
        let workspace_id = self.active_workspace(user_id).await?;
        self.workspaces
            .check_permission(&workspace_id, user_id, WorkspaceRole::ADMIN)
            .await?;
        self.repository.create(&workspace_id, request).await
    }

    pub async fn list(&self, user_id: &str) -> Result<Vec<Connection>> {
        let workspace_id = self.active_workspace(user_id).await?;
        self.workspaces
            .check_permission(&workspace_id, user_id, WorkspaceRole::VIEWER)
            .await?;
        self.repository.get_all(&workspace_id).await
    }

    pub async fn get(&self, connection_id: &str, user_id: &str) -> Result<Option<Connection>> {
        match self
            .authorize(connection_id, user_id, WorkspaceRole::VIEWER)
            .await
        {
            Ok(connection) => Ok(Some(connection)),
            Err(error) if error.to_string() == "Connection not found" => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub async fn update(
        &self,
        connection_id: &str,
        user_id: &str,
        request: UpdateConnection,
    ) -> Result<Connection> {
        self.authorize(connection_id, user_id, WorkspaceRole::ADMIN)
            .await?;
        self.repository.update(connection_id, request).await
    }

    pub async fn delete(&self, connection_id: &str, user_id: &str) -> Result<u64> {
        self.authorize(connection_id, user_id, WorkspaceRole::ADMIN)
            .await?;
        self.repository.delete(connection_id).await
    }
}

#[tracing::instrument(skip(pool), err)]
pub async fn start_health_check_service(pool: &deadpool_redis::Pool) -> Result<bool> {
    tracing::debug!("checking Redis health");
    let mut connection = match timeout(Duration::from_secs(1), pool.get()).await {
        Ok(Ok(connection)) => connection,
        _ => {
            tracing::warn!("Redis health check could not acquire a connection");
            return Ok(false);
        }
    };
    Ok(matches!(
        timeout(
            Duration::from_secs(1),
            redis::cmd("PING").query_async::<String>(&mut *connection)
        )
        .await,
        Ok(Ok(_))
    ))
}

#[tracing::instrument(skip(redis_url), err)]
pub async fn test_redis_connection_service(redis_url: String) -> Result<bool, String> {
    tracing::debug!("testing Redis connection");
    let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
    let mut connection = tokio::time::timeout(
        Duration::from_secs(5),
        client.get_multiplexed_async_connection(),
    )
    .await
    .map_err(|_| "Timeout".to_string())
    .and_then(|r| r.map_err(|e| e.to_string()))?;
    let response: String = redis::cmd("PING")
        .query_async(&mut connection)
        .await
        .map_err(|e| e.to_string())?;
    Ok(response == "PONG")
}

#[tracing::instrument(skip(data), fields(host = %data.host, port = data.port), err)]
pub async fn check_redis_version(data: &CreateConnection) -> Result<()> {
    tracing::debug!("checking Redis server version");
    let url = create_redis_url(
        &data.host,
        data.port,
        data.username.as_deref(),
        data.password.as_deref(),
        data.db.unwrap_or(0),
        data.is_tls_enabled,
    )?;
    let client = redis::Client::open(url).context("Failed to create Redis client")?;
    let info: String = timeout(Duration::from_secs(5), async {
        let mut connection = client.get_multiplexed_async_connection().await?;
        redis::cmd("INFO")
            .arg("server")
            .query_async(&mut connection)
            .await
    })
    .await
    .map_err(|_| anyhow::anyhow!("Connection timed out after 5 seconds"))??;
    let version = info
        .lines()
        .find_map(|line| line.strip_prefix("redis_version:"))
        .context("Could not find redis_version in INFO output")?
        .trim();
    if Version::parse(version)? < Version::parse("6.2.0")? {
        bail!(
            "Redis version {} is unsupported. Please use Redis 6.2 or higher.",
            version
        );
    }
    Ok(())
}

pub fn create_redis_url(
    host: &str,
    port: i32,
    username: Option<&str>,
    password: Option<&str>,
    db: i32,
    tls: bool,
) -> Result<String> {
    let scheme = if tls { "rediss" } else { "redis" };
    let mut url = url::Url::parse(&format!("{scheme}://{host}:{port}/{db}"))?;
    if let Some(username) = username {
        url.set_username(username)
            .map_err(|_| anyhow::anyhow!("Invalid Redis username"))?;
    }
    if let Some(password) = password {
        url.set_password(Some(password))
            .map_err(|_| anyhow::anyhow!("Invalid Redis password"))?;
    }
    Ok(url.to_string())
}
