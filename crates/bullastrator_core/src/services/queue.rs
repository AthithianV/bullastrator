use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use bullastrator_storage::repositories::QueueRepository;

use super::RedisConnection;

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
}

impl QueueService {
    pub fn new(redis: RedisConnection, repository: QueueRepository) -> Self {
        Self { redis, repository }
    }

    pub async fn get_all_bullmq_queues(&self) -> Result<Vec<String>> {
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

    pub async fn pause_queue_service(
        &self,
        queue_name: &str,
        should_pause: bool,
    ) -> Result<String> {
        let queue = self.redis.queue(queue_name).await?;
        if should_pause {
            queue.pause().await?;
        } else {
            queue.resume().await?;
        }
        Ok(if should_pause { "paused" } else { "resumed" }.into())
    }

    pub async fn get_queue_details_service(&self, queue_name: &str) -> Result<QueueDetails> {
        let queue = self.redis.queue(queue_name).await?;
        let workers = queue.get_workers_count().await?;
        Ok(QueueDetails {
            name: queue_name.into(),
            is_paused: queue.is_paused().await?,
            version: queue.get_version().await?.unwrap_or_else(|| "unknown".into()),
            prefix: self.redis.prefix.clone(),
            active_workers: workers,
        })
    }
}
