use anyhow::Result;
use bullastrator_core::services::{queue::QueueService, user::UserService, workspace::WorkspaceService, RedisConnection};
use bullastrator_storage::repositories::{QueueRepository, UserRepository};
use deadpool_redis::{Config as RedisConfig, Runtime};
use sqlx::SqlitePool;

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub redis: RedisConnection,
    pub users: UserService,
    pub queues: QueueService,
    pub workspaces: WorkspaceService,
}

impl AppState {
    pub fn new(db: SqlitePool, redis_url: &str, connection_id: &str, prefix: &str) -> Result<Self> {
        let redis_pool = RedisConfig::from_url(redis_url).create_pool(Some(Runtime::Tokio1))?;
        let redis = RedisConnection::with_url(redis_pool, connection_id, prefix, redis_url);
        Ok(Self {
            users: UserService::new(UserRepository::new(db.clone())),
            queues: QueueService::new(redis.clone(), QueueRepository::new(db.clone())),
            workspaces: WorkspaceService::new(bullastrator_storage::repositories::WorkspaceRepository::new(db.clone())),
            db,
            redis,
        })
    }
}

pub fn router(state: AppState) -> axum::Router {
    routes::router(state)
}

pub async fn serve(state: AppState, address: std::net::SocketAddr) -> Result<()> {
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, router(state)).await?;
    Ok(())
}
