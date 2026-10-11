use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::bull_drivers::bullmq_driver_v5::BullmqV5Driver;
use crate::bull_drivers::driver_registry::BullmqDriver;

use bullastrator_storage::repositories::QueueRepository;

use super::RedisConnection;
use super::connection::ConnectionService;
use bullastrator_storage::models::WorkspaceRole;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueueDetails {
    pub name: String,
    pub is_paused: bool,
    pub version: String,
    pub prefix: String,
    pub active_workers: usize,
}

#[derive(Clone)]
pub struct QueueService {
    redis: RedisConnection,
    repository: QueueRepository,
    connections: ConnectionService,
}

impl QueueService {
    pub fn new(
        redis: RedisConnection,
        repository: QueueRepository,
        connections: ConnectionService,
    ) -> Self {
        Self {
            redis,
            repository,
            connections,
        }
    }

    async fn authorize(&self, user_id: &str, role: WorkspaceRole) -> Result<()> {
        self.connections
            .authorize(&self.redis.connection_id, user_id, role)
            .await
            .map(|_| ())
    }

    #[tracing::instrument(skip(self), fields(connection_id = %self.redis.connection_id), err)]
    pub async fn sync_queues(&self, user_id: &str) -> Result<Vec<String>> {
        self.authorize(user_id, WorkspaceRole::ADMIN).await?;
        tracing::debug!("discovering BullMQ queues");
        let mut connection = self.redis.pool.get().await.context("Connection failed")?;
        let pattern = format!("{}:*:meta", self.redis.prefix);
        let mut cursor = 0_u64;
        let mut names = std::collections::HashSet::new();
        loop {
            let (next, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(&pattern)
                .arg("COUNT")
                .arg(5000)
                .query_async(&mut *connection)
                .await?;
            for key in keys {
                if let Some(name) = key
                    .strip_prefix(&format!("{}:", self.redis.prefix))
                    .and_then(|v| v.strip_suffix(":meta"))
                {
                    names.insert(name.to_string());
                }
            }
            cursor = next;
            if cursor == 0 {
                break;
            }
        }
        let mut result: Vec<_> = names.into_iter().collect();
        result.sort();
        self.repository
            .sync_all_queues(&self.redis.connection_id, &result, true)
            .await?;
        Ok(result)
    }

    #[tracing::instrument(skip(self), fields(connection_id = %self.redis.connection_id, queue = %queue_name, paused = should_pause), err)]
    pub async fn pause_queue_service(
        &self,
        queue_name: &str,
        should_pause: bool,
        user_id: &str,
    ) -> Result<String> {
        self.authorize(user_id, WorkspaceRole::EDITOR).await?;
        tracing::info!("updating queue pause state");

        let arg = if should_pause { "paused" } else { "resumed" };

        let (script_content, key_gen) = BullmqV5Driver.pause();
        let keys = key_gen(&self.redis.prefix, &queue_name, should_pause);
        let script = redis::Script::new(script_content);
        let mut redis_conn = self
            .redis
            .pool
            .get()
            .await
            .context("Failed to get Redis connection")?;

        let result: redis::RedisResult<Option<i32>> = script
            .key(keys)
            .arg(arg)
            .invoke_async(&mut redis_conn)
            .await;

        let _: Option<i32> = result.map_err(|error| {
            tracing::error!(?error, queue_name, "Redis pause/resume failed");
            error
        })?;

        Ok(arg.to_string())
    }

    #[tracing::instrument(skip(self), fields(connection_id = %self.redis.connection_id, queue = %queue_name), err)]
    pub async fn get_queue_details_service(
        &self,
        queue_name: &str,
        user_id: &str,
    ) -> Result<QueueDetails> {
        self.authorize(user_id, WorkspaceRole::VIEWER).await?;
        tracing::debug!("loading queue details");
        let queue = self.redis.queue(queue_name).await?;
        let workers = queue.get_workers_count().await?;
        Ok(QueueDetails {
            name: queue_name.into(),
            is_paused: queue.is_paused().await?,
            version: queue
                .get_version()
                .await?
                .unwrap_or_else(|| "unknown".into()),
            prefix: self.redis.prefix.clone(),
            active_workers: workers,
        })
    }
}
