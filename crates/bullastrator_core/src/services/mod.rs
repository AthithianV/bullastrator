pub mod connection;
pub mod job;
pub mod queue;
pub mod tab;
pub mod user;
pub mod workspace;

use deadpool_redis::Pool;

#[derive(Clone)]
pub struct RedisConnection {
    pub pool: Pool,
    pub prefix: String,
    pub connection_id: String,
    pub bullmq_options: bullmq::options::RedisConnectionOptions,
}

impl RedisConnection {
    pub fn new(pool: Pool, connection_id: impl Into<String>, prefix: impl Into<String>) -> Self {
        Self::with_options(
            pool,
            connection_id,
            prefix,
            bullmq::options::RedisConnectionOptions::default(),
        )
    }

    pub fn with_options(
        pool: Pool,
        connection_id: impl Into<String>,
        prefix: impl Into<String>,
        bullmq_options: bullmq::options::RedisConnectionOptions,
    ) -> Self {
        Self {
            pool,
            connection_id: connection_id.into(),
            prefix: prefix.into(),
            bullmq_options,
        }
    }

    pub async fn queue(&self, queue_name: &str) -> anyhow::Result<bullmq::Queue> {
        use bullmq::options::QueueOptions;

        let options = QueueOptions::new()
            .connection(self.bullmq_options.clone())
            .prefix(self.prefix.clone())
            .skip_version_check();

        Ok(bullmq::Queue::with_options(queue_name, options).await?)
    }
}
