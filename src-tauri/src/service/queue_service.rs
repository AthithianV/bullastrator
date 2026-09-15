use std::collections::HashMap;

use crate::{
    bull_drivers::{bullmq_driver_v5::BullmqV5Driver, driver_registry::BullmqDriver},
    model::queue_model::QueueDetails,
    repository::queue_repository::QueueRepository,
    utilities::app_state::ConnectionPool,
};
use anyhow::{Context, Result};
use sea_orm::DatabaseConnection;

pub struct QueueService<'a> {
    connection_pool: &'a ConnectionPool,
    prefix: &'a str,
    bull_driver: &'a BullmqV5Driver,
}

impl<'a> QueueService<'a> {
    pub fn new(connection_pool: &'a ConnectionPool) -> Self {
        Self {
            connection_pool,
            prefix: connection_pool.metadata.bullmq_prefix.as_str(),
            bull_driver: &BullmqV5Driver,
        }
    }

    pub async fn get_all_bullmq_queues(&self, db_conn: DatabaseConnection) -> Result<Vec<String>> {
        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        // Step 3: Now pass all queues to the QueueReposiroty.sync_queues
        let queue_repository = QueueRepository::new(&db_conn);

        let prefix = &self.prefix;
        let search_pattern = format!("{}:*:meta", prefix);

        let mut queue_names = std::collections::HashSet::new();
        let mut cursor: u64 = 0;

        let script_content = self.bull_driver.get_queues();
        let script = redis::Script::new(script_content);

        let db_size: u64 = redis::cmd("DBSIZE").query_async(&mut redis_conn).await?;

        // Use 10% of keyspace per iteration, capped between 1000 and 50000
        let count = (db_size / 10).clamp(1000, 50000);

        loop {
            // Build the command manually to include COUNT
            let result: Vec<redis::Value> = script
                .arg(cursor)
                .arg(&search_pattern)
                .arg(count)
                .invoke_async(&mut redis_conn)
                .await?;

            let next_cursor: u64 = redis::from_redis_value(&result[0])?;
            let batch: Vec<String> = redis::from_redis_value(&result[1])?;

            if !batch.is_empty() {
                for name in &batch {
                    queue_names.insert(name.clone());
                }

                queue_repository
                    .sync_all_queues(self.connection_pool.metadata.id, &vec![], false)
                    .await?;
            }

            cursor = next_cursor;
            if cursor == 0 {
                break;
            }
        }

        let mut result: Vec<String> = queue_names.into_iter().collect();
        result.sort();

        queue_repository
            .sync_all_queues(self.connection_pool.metadata.id, &result, true)
            .await?;

        Ok(result)
    }

    pub async fn pause_queue_service(
        &self,
        queue_name: String,
        should_pause: bool,
    ) -> Result<String> {
        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        let arg = if should_pause { "paused" } else { "resumed" };

        let (script_content, key_gen) = self.bull_driver.pause();
        let keys = key_gen(self.prefix, &queue_name, should_pause);
        let script = redis::Script::new(script_content);

        let _: Option<i32> = script
            .key(keys)
            .arg(arg) // ARGV[1]
            .invoke_async(&mut redis_conn)
            .await
            .context("Failed to pause/resume Queue")?;

        Ok(arg.to_string())
    }

    pub async fn get_queue_details_service(&self, queue_name: String) -> Result<QueueDetails> {
        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        let prefix = format!("{}:{}", self.prefix, queue_name);
        let meta_key = format!("{}:meta", prefix);

        let mut pipe = redis::pipe();
        pipe.hgetall(&meta_key) // Get all metadata fields
            .scard(format!("{}:workers", prefix));

        let (meta, worker_count): (HashMap<String, String>, usize) =
            pipe.query_async(&mut redis_conn).await?;

        // 2. Extract specific values from metadata
        let is_paused = meta.get("paused").map(|v| v == "1").unwrap_or(false);
        let bull_version = meta
            .get("version")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        let prefix = meta
            .get("prefix")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());

        Ok(QueueDetails {
            name: queue_name,
            prefix,
            is_paused,
            version: bull_version,
            active_workers: worker_count,
        })
    }
}
