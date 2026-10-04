pub mod connection;
pub mod job;
pub mod queue;
pub mod workspace;
pub mod user;

use deadpool_redis::Pool;

#[derive(Clone)]
pub struct RedisConnection {
    pub pool: Pool,
    pub prefix: String,
    pub connection_id: String,
    /// URL used by bullmq-official. The deadpool connection remains available
    /// for the custom Lua operations owned by bullastrator_core.
    pub redis_url: String,
}

impl RedisConnection {
    pub fn new(pool: Pool, connection_id: impl Into<String>, prefix: impl Into<String>) -> Self {
        Self::with_url(
            pool,
            connection_id,
            prefix,
            "redis://127.0.0.1:6379".to_string(),
        )
    }

    pub fn with_url(
        pool: Pool,
        connection_id: impl Into<String>,
        prefix: impl Into<String>,
        redis_url: impl Into<String>,
    ) -> Self {
        Self {
            pool,
            connection_id: connection_id.into(),
            prefix: prefix.into(),
            redis_url: redis_url.into(),
        }
    }

    pub async fn queue(&self, queue_name: &str) -> anyhow::Result<bullmq::Queue> {
        use bullmq::options::{QueueOptions, RedisConnectionOptions};

        let options = QueueOptions::new()
            .connection(RedisConnectionOptions {
                url: self.redis_url.clone(),
                ..Default::default()
            })
            .prefix(self.prefix.clone())
            .skip_version_check();

        Ok(bullmq::Queue::with_options(queue_name, options).await?)
    }
}
