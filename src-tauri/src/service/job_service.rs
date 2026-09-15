use anyhow::{Context, Result};
use deadpool_redis::Connection;
use redis::{AsyncCommands, Script};
use serde_json::{Value, to_string};
use std::collections::HashMap;

use crate::bull_drivers::bullmq_driver_v5::BullmqV5Driver;
use crate::bull_drivers::driver_registry::BullmqDriver;
use crate::model::job_model::{AddJobModel, Job, JobDataResponse, JobStatus, RetryJobStatus};
use crate::model::job_model::{JobDetails, JobFilterType, JobSearchResult, RetryStrategy};

use crate::model::queue_model::QueueJobCounts;
use crate::utilities::app_state::ConnectionPool;

pub struct JobService<'a> {
    connection_pool: &'a ConnectionPool,
    prefix: &'a str,
    bull_driver: &'a BullmqV5Driver,
}

impl<'a> JobService<'a> {
    pub fn new(connection_pool: &'a ConnectionPool) -> Self {
        Self {
            connection_pool,
            prefix: connection_pool.metadata.bullmq_prefix.as_str(),
            bull_driver: &BullmqV5Driver,
        }
    }

    pub async fn get_jobs_in_queue_service(
        &self,
        queue_name: String,
        status: JobStatus,
        cursor: usize,
        limit: usize,
    ) -> Result<JobSearchResult> {
        let mut conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        let prefix = self.prefix;
        let status_suffix = status.redis_key_suffix();
        let job_key_prefix = format!("{}:{}:", prefix, queue_name);

        // Map status to commands
        let (range_cmd, count_cmd) = match status {
            JobStatus::Wait | JobStatus::Active | JobStatus::Paused => ("LRANGE", "LLEN"),
            JobStatus::Completed | JobStatus::Failed => ("ZREVRANGE", "ZCARD"),
            JobStatus::Delayed | JobStatus::Prioritized => ("ZRANGE", "ZCARD"),
        };

        let (script, key_gen) = self.bull_driver.get();
        let key = key_gen(prefix, &queue_name, status_suffix);

        let start = cursor;
        let end = cursor + limit - 1;

        let script = redis::Script::new(script);
        let (total_count, raw_jobs): (usize, Vec<(String, Vec<Option<String>>)>) = script
            .key(key)
            .arg(range_cmd)
            .arg(count_cmd)
            .arg(start)
            .arg(end)
            .arg(job_key_prefix)
            .invoke_async(&mut *conn)
            .await?;

        let jobs: Vec<Job> = raw_jobs
            .into_iter()
            .filter_map(|(id, map)| {
                if map.is_empty() || map.first().and_then(|v| v.as_ref()).is_none() {
                    return None;
                }
                Some(Job::from_hmget_values(id, &map))
            })
            .collect();

        let scanned_count = start + jobs.len();
        Ok(JobSearchResult {
            jobs,
            next_cursor: scanned_count,
            has_more: scanned_count < total_count,
            scanned_count,
        })
    }

    pub async fn get_job_logs_service(
        &self,
        queue_name: String,
        job_id: String,
    ) -> Result<Vec<String>> {
        let mut conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        let prefix = self.prefix;

        let (script, key_gen) = self.bull_driver.get_job_logs();
        let key = key_gen(prefix, &queue_name, &job_id);

        let script = redis::Script::new(script);
        let logs: Vec<String> = script.key(key).invoke_async(&mut *conn).await?;
        Ok(logs)
    }

    pub async fn check_if_job_exists_in_a_status(
        &self,
        prefix: &str,
        queue_name: &str,
        job_id: &str,
        status: JobStatus,
        redis_conn: &mut Connection,
    ) -> Result<bool> {
        let status_key = format!("{}:{}:{}", prefix, queue_name, status.redis_key_suffix());

        let is_in_state = match status {
            JobStatus::Completed
            | JobStatus::Failed
            | JobStatus::Delayed
            | JobStatus::Prioritized => {
                // zscore returns a double (f64) if the member exists
                let z_exists: Option<f64> = redis_conn.zscore(&status_key, job_id).await.ok();

                // sismember returns a 0 or 1 (bool)
                let s_exists: bool = redis_conn
                    .sismember(&status_key, job_id)
                    .await
                    .unwrap_or(false);

                z_exists.is_some() || s_exists
            }
            _ => {
                let list_pos: Option<i64> = redis_conn
                    .lpos(&status_key, job_id, Default::default())
                    .await?;
                list_pos.is_some()
            }
        };

        Ok(is_in_state)
    }

    pub async fn get_jobs_by_id_service(
        &self,
        queue_name: String,
        job_id: String,
        status: JobStatus,
    ) -> Result<Option<JobDetails>> {
        #[cfg(debug_assertions)]
        {
            use std::{thread, time::Duration};

            println!("Dev mode: Sleeping for 2 second...");
            thread::sleep(Duration::from_secs(2));
        }

        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        let prefix = self.prefix;
        let job_key = format!("{}:{}:{}", prefix, queue_name, job_id);

        let is_in_state = self
            .check_if_job_exists_in_a_status(prefix, &queue_name, &job_id, status, &mut redis_conn)
            .await?;

        if !is_in_state {
            return Ok(None);
        }

        let map: HashMap<String, String> = redis_conn
            .hgetall(&job_key)
            .await
            .context(format!("Failed to fetch job {} from Redis", job_id))?;

        if map.is_empty() {
            return Ok(None);
        }

        Ok(Some(JobDetails::from_redis_map(job_id, &map)))
    }

    pub async fn get_job_data(
        &self,
        queue_name: String,
        job_ids: Vec<String>,
        status: JobStatus,
        start_opt: Option<i32>,
        end_opt: Option<i32>,
    ) -> Result<Vec<serde_json::Value>> {
        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        let prefix = self.prefix;

        let (range_cmd, count_cmd) = match status {
            JobStatus::Wait | JobStatus::Active | JobStatus::Paused => ("LRANGE", "LLEN"),
            JobStatus::Completed | JobStatus::Failed => ("ZREVRANGE", "ZCARD"),
            JobStatus::Delayed | JobStatus::Prioritized => ("ZRANGE", "ZCARD"),
        };

        let start = start_opt.unwrap_or(0);
        let end = end_opt.unwrap_or(start + 99);

        let (script_content, key_gen) = self.bull_driver.get_job_data();

        let state_key = key_gen(prefix, &queue_name, status.redis_key_suffix());

        let job_key_prefix = format!("{}:{}:", prefix, queue_name);

        let script = Script::new(script_content);

        script
            .load_async(&mut *redis_conn)
            .await
            .context("Failed to load script")?;

        let mut all_jobs: Vec<serde_json::Value> = Vec::new();

        // ============================================================
        // SELECTED JOBS
        // ============================================================

        if !job_ids.is_empty() {
            for chunk in job_ids.chunks(100) {
                let mut invocation = script.prepare_invoke();

                invocation.key(&state_key);

                invocation.arg(range_cmd);
                invocation.arg(count_cmd);
                invocation.arg(start);
                invocation.arg(end);
                invocation.arg(&job_key_prefix);

                for id in chunk {
                    invocation.arg(id);
                }

                let json: String = invocation
                    .invoke_async(&mut *redis_conn)
                    .await
                    .context("Failed to execute job data script")?;

                let response: JobDataResponse = serde_json::from_str(&json)
                    .context("Invalid JSON returned from Redis script")?;

                for raw_job in response.jobs {
                    let job: Value = serde_json::from_str(&raw_job).context("Invalid job JSON")?;

                    all_jobs.push(job);
                }
            }
        }
        // ============================================================
        // RANGE
        // ============================================================
        else {
            let mut current_start = start;

            while current_start <= end {
                let current_end = std::cmp::min(current_start + 99, end);

                let mut invocation = script.prepare_invoke();

                invocation.key(&state_key);

                invocation.arg(range_cmd);
                invocation.arg(count_cmd);
                invocation.arg(current_start);
                invocation.arg(current_end);
                invocation.arg(&job_key_prefix);

                let json: String = invocation
                    .invoke_async(&mut *redis_conn)
                    .await
                    .context("Failed to execute job data script")?;

                let response: JobDataResponse = serde_json::from_str(&json)
                    .context("Invalid JSON returned from Redis script")?;

                for raw_job in response.jobs {
                    let job: Value = serde_json::from_str(&raw_job).context("Invalid job JSON")?;

                    all_jobs.push(job);
                }

                current_start = current_end + 1;
            }
        }

        Ok(all_jobs)
    }

    pub async fn search_jobs_in_queue_service(
        &self,
        queue_name: String,
        status: JobStatus,
        filters: Vec<JobFilterType>,
        cursor: usize,
        _limit: usize,
    ) -> Result<JobSearchResult> {
        #[cfg(debug_assertions)]
        {
            use std::{thread, time::Duration};

            println!("Dev mode: Sleeping for 2 second...");
            thread::sleep(Duration::from_secs(2));
        }

        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        let prefix = self.prefix;
        let job_prefix = format!("{}:{}:", prefix, queue_name);

        // --- 1. DIRECT ID LOOKUP (Fast Path) ---
        if let Some(target_id) = filters.iter().find_map(|f| match f {
            JobFilterType::JobId(id) => Some(id),
            _ => None,
        }) {
            let job_key = format!("{}:{}:{}", prefix, queue_name, target_id);

            let is_in_state = self
                .check_if_job_exists_in_a_status(
                    prefix,
                    &queue_name,
                    target_id,
                    status,
                    &mut redis_conn,
                )
                .await?;

            let map: HashMap<String, String> = redis_conn.hgetall(&job_key).await?;

            if map.is_empty() || !is_in_state {
                return Ok(JobSearchResult {
                    jobs: vec![],
                    next_cursor: 0,
                    has_more: false,
                    scanned_count: 0,
                });
            }

            return Ok(JobSearchResult {
                jobs: vec![Job::from_map(target_id.clone(), &map)],
                next_cursor: 0,
                has_more: false,
                scanned_count: 1,
            });
        }

        // --- 2. EXTRACT FILTERS FOR LUA ---
        let mut data_kw = String::new();
        let mut reason_kw = String::new();

        for filter in &filters {
            match filter {
                JobFilterType::Keyword(kw) => data_kw = kw.clone(),
                JobFilterType::FailedReason(reason) => reason_kw = reason.clone(),
                _ => {}
            }
        }

        let is_zset = match status {
            JobStatus::Wait | JobStatus::Active | JobStatus::Paused => "0",
            _ => "1",
        };

        // --- 3. SCANNING LOGIC ---
        let mut matches = Vec::new();

        // SAFE BATCH SIZE: Balances speed with preventing Redis blocking
        let batch_size = 500;
        let start = cursor;
        let end = start + batch_size - 1;

        let (script_content, key_gen) = self.bull_driver.search();

        let script = Script::new(script_content);
        let key = key_gen(prefix, &queue_name, status);

        // A. Execute Lua Script
        // This returns ONLY the IDs that matched our criteria
        let result: Vec<redis::Value> = script
            .key(&key)
            .arg(is_zset)
            .arg(start)
            .arg(end)
            .arg(&job_prefix)
            .arg(&data_kw)
            .arg(&reason_kw)
            .invoke_async(&mut redis_conn)
            .await?;

        // Parse the complex return from Lua
        let matched_ids: Vec<String> = redis::from_redis_value(&result[0])?;
        let actual_scanned_count: usize = redis::from_redis_value(&result[1])?;

        if !matched_ids.is_empty() {
            let mut pipe = redis::pipe();
            for job_id in &matched_ids {
                let job_key = format!("{}{}", job_prefix, job_id);
                pipe.cmd("HGETALL").arg(job_key);
            }

            let job_maps: Vec<HashMap<String, String>> = pipe.query_async(&mut redis_conn).await?;

            for (id, map) in matched_ids.into_iter().zip(job_maps.into_iter()) {
                if !map.is_empty() {
                    matches.push(Job::from_map(id, &map));
                }
            }
        }

        let next_cursor = cursor + actual_scanned_count;

        let hit_match_limit = matches.len() >= 50;
        let exhausted_batch = actual_scanned_count == batch_size;
        let has_more = hit_match_limit || exhausted_batch;

        Ok(JobSearchResult {
            jobs: matches,
            next_cursor,
            has_more,
            scanned_count: actual_scanned_count,
        })
    }

    pub async fn get_job_count_in_queue_service(
        &self,
        queue_names: Vec<&str>,
    ) -> Result<Vec<QueueJobCounts>> {
        #[cfg(debug_assertions)]
        {
            use std::{thread, time::Duration};

            println!("Dev mode: Sleeping for 2 second...");
            thread::sleep(Duration::from_secs(2));
        }

        let prefix = self.prefix;

        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        let script_content = self.bull_driver.get_counts();

        let script = Script::new(script_content);

        let mut all_counts = Vec::with_capacity(queue_names.len());

        // Batch in chunks of 100
        for chunk in queue_names.chunks(100) {
            let chunk_vec: Vec<&str> = chunk.to_vec();

            let mut invocation = script.prepare_invoke();

            invocation.key(prefix);

            // 4. Pass the queue names to the ARGV table in Lua
            // Your Lua script uses ARGV to loop and identify the queue names
            for name in &chunk_vec {
                invocation.arg(name);
            }

            // Execute this specific batch
            let raw_data: Vec<Vec<redis::Value>> =
                invocation
                    .invoke_async(&mut redis_conn)
                    .await
                    .context("Lua script execution failed for batch")?;

            // Map and append to our main result vector
            let chunk_results = raw_data
                .into_iter()
                .filter(|row| row.len() >= 8)
                .map(|row| {
                    let wait: i64 = redis::from_redis_value(&row[1]).unwrap_or(0);
                    let active: i64 = redis::from_redis_value(&row[2]).unwrap_or(0);
                    let completed: i64 = redis::from_redis_value(&row[3]).unwrap_or(0);
                    let failed: i64 = redis::from_redis_value(&row[4]).unwrap_or(0);
                    let delayed: i64 = redis::from_redis_value(&row[5]).unwrap_or(0);
                    let paused: i64 = redis::from_redis_value(&row[6]).unwrap_or(0);
                    let prioritized: i64 = redis::from_redis_value(&row[7]).unwrap_or(0);

                    // Sum them up here
                    let total = wait + active + completed + failed + delayed + paused;

                    QueueJobCounts {
                        queue_name: redis::from_redis_value(&row[0])
                            .unwrap_or_else(|_| "unknown".to_string()),
                        wait,
                        active,
                        completed,
                        failed,
                        delayed,
                        paused,
                        prioritized,
                        total,
                    }
                });

            all_counts.extend(chunk_results);
        }

        Ok(all_counts)
    }

    pub async fn add_job_to_queue_service(
        &self,
        queue_name: String,
        jobs: Vec<AddJobModel>,
    ) -> Result<Vec<String>> {
        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        if jobs.is_empty() {
            return Ok(Vec::new());
        }

        let first_job_opts = &jobs[0].opts;
        let (script_content, key_gen) = if first_job_opts.delay.unwrap_or(0) > 0 {
            self.bull_driver.add_delayed()
        } else if first_job_opts.priority.unwrap_or(0) > 0 {
            self.bull_driver.add_prioritized()
        } else {
            self.bull_driver.add()
        };
        let keys = key_gen(self.prefix, &queue_name);

        let script = Script::new(script_content);

        // Pre-load the script into Redis memory to ensure we use EVALSHA in the pipeline
        script
            .load_async(&mut *redis_conn)
            .await
            .context("Failed to load script")?;
        let mut all_job_ids = Vec::with_capacity(jobs.len());

        for chunk in jobs.chunks(200) {
            let mut pipe = redis::pipe();

            for job in chunk {
                let args = self.bull_driver.add_job_args(
                    self.prefix,
                    &queue_name,
                    &job.name,
                    &job.data,
                    &job.opts,
                );

                let mut invocation = script.prepare_invoke();
                for key in &keys {
                    invocation.key(key);
                }
                for arg in &args {
                    invocation.arg(arg);
                }

                pipe.invoke_script(&invocation);
            }

            // Execute the chunk
            let results: Vec<String> = pipe.query_async(&mut *redis_conn).await?;
            all_job_ids.extend(results);
        }

        Ok(all_job_ids)
    }

    pub async fn update_job_data_service(
        &self,
        queue_name: String,
        job_id: String,
        job_data: Value,
    ) -> Result<String> {
        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        let (script_content, key_gen) = self.bull_driver.update();
        let script = Script::new(script_content);

        let key = key_gen(self.prefix, &queue_name, &job_id);

        // 2. Serialize Data and Options
        let data_json = to_string(&job_data).unwrap_or("{}".to_string());

        // 3. Invoke the Script
        let result: String = script
            .key(key)
            .arg(data_json)
            .invoke_async(&mut redis_conn)
            .await?;

        Ok(result)
    }

    pub async fn retry_failed_jobs_service(
        &self,
        queue_name: String,
        job_ids: Vec<String>,
        strategy: RetryStrategy,
        retry_job_status: &RetryJobStatus,
    ) -> Result<Vec<String>> {
        #[cfg(debug_assertions)]
        {
            use std::{thread, time::Duration};

            println!("Dev mode: Sleeping for 2 second...");
            thread::sleep(Duration::from_secs(2));
        }

        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        let (script_content, key_gen, arg_gen) = match retry_job_status {
            RetryJobStatus::Completed => self.bull_driver.reprocess(),
            RetryJobStatus::Failed => self.bull_driver.retry(),
        };

        let script = Script::new(script_content);

        // Pre-load to ensure performance
        script.load_async(&mut *redis_conn).await?;

        let mut all_results = Vec::with_capacity(job_ids.len());

        // 1. Chunk the retries (300 is a safe middle ground)
        for chunk in job_ids.chunks(300) {
            let mut pipe = redis::pipe();

            for job_id in chunk {
                let keys = key_gen(self.prefix, &queue_name, job_id);
                let args = arg_gen(
                    job_id,
                    match strategy {
                        RetryStrategy::ToBack => false,
                        RetryStrategy::ToFront => true,
                    },
                );

                let mut invocation = script.prepare_invoke();

                // Define KEYS
                for key in &keys {
                    invocation.key(key);
                }

                // Define ARGV
                for arg in &args {
                    invocation.arg(arg);
                }

                pipe.invoke_script(&invocation);
            }

            // 2. Execute chunk and map result
            // BullMQ scripts usually return 1 for success or a negative code for failure
            let chunk_results: Vec<i64> = pipe.query_async(&mut *redis_conn).await?;
            all_results.extend(chunk_results.iter().map(|r| r.to_string()));
        }

        Ok(all_results)
    }

    pub async fn retry_all_failed_jobs_service(
        &self,
        queue_name: String,
        strategy: RetryStrategy,
        retry_job_status: &RetryJobStatus,
    ) -> Result<String> {
        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        let raw_prefix = self.prefix.trim_end_matches(':');
        let job_status = retry_job_status.redis_key_suffix();
        let failed_key = format!("{}:{}:{}", raw_prefix, queue_name, job_status);
        let chunk_size = 1000;
        let mut total_retried = 0;

        loop {
            // 1. Fetch a batch of IDs (Always 0 to chunk_size because the list shrinks!)
            let job_ids: Vec<String> = redis_conn.zrange(&failed_key, 0, chunk_size - 1).await?;

            if job_ids.is_empty() {
                break; // No more jobs to retry
            }

            // 2. Reuse your existing pipeline logic
            let _ = self
                .retry_failed_jobs_service(
                    queue_name.clone(),
                    job_ids.clone(),
                    strategy.clone(),
                    &retry_job_status,
                )
                .await?;

            total_retried += job_ids.len();
        }

        Ok(format!("Successfully retried {} jobs", total_retried))
    }

    pub async fn promote_jobs_service(
        &self,
        queue_name: String,
        job_ids: Vec<String>,
    ) -> Result<Vec<String>> {
        #[cfg(debug_assertions)]
        {
            use std::{thread, time::Duration};

            println!("Dev mode: Sleeping for 2 second...");
            thread::sleep(Duration::from_secs(2));
        }

        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        // 1. Prepare Script via Driver
        let (script_content, key_gen) = self.bull_driver.promote();
        let script = Script::new(script_content);

        // Pre-load the script to ensure the pipe uses EVALSHA internally
        script.load_async(&mut *redis_conn).await?;

        let prefix = format!(
            "{}:{}:",
            self.connection_pool.metadata.bullmq_prefix, queue_name
        );

        let mut all_results = Vec::with_capacity(job_ids.len());

        // 2. Chunk the promotions (300 is efficient for ZSET/LIST moves)
        for chunk in job_ids.chunks(300) {
            let mut pipe = redis::pipe();

            for job_id in chunk {
                // Generate the 9 keys required by the promote script
                let keys = key_gen(self.prefix, &queue_name);
                let mut invocation = script.prepare_invoke();

                for key in &keys {
                    invocation.key(key);
                }

                // Define ARGV based on the BullMQ promote script header
                invocation
                    .arg(&prefix) // ARGV[1]
                    .arg(job_id); // ARGV[2]

                pipe.invoke_script(&invocation);
            }

            // 3. Execute and map
            // Results: 0 = Success, -3 = Job not found in delayed set
            let chunk_results: Vec<i64> = pipe.query_async(&mut *redis_conn).await?;
            all_results.extend(chunk_results.iter().map(|r| r.to_string()));
        }

        Ok(all_results)
    }

    pub async fn promote_all_jobs_service(&self, queue_name: String) -> Result<String> {
        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        let raw_prefix = self.prefix.trim_end_matches(':');
        let deleyed_key = format!("{}:{}:delayed", raw_prefix, queue_name);
        let chunk_size = 1000;
        let mut total_retried = 0;

        loop {
            let job_ids: Vec<String> = redis_conn.zrange(&deleyed_key, 0, chunk_size - 1).await?;

            if job_ids.is_empty() {
                break;
            }

            let _ = self
                .promote_jobs_service(queue_name.clone(), job_ids.clone())
                .await?;

            total_retried += job_ids.len();
        }

        Ok(format!("Successfully retried {} jobs", total_retried))
    }

    pub async fn delete_jobs_service(
        &self,
        queue_name: String,
        job_ids: Vec<String>,
        remove_children: bool,
    ) -> Result<Vec<String>> {
        #[cfg(debug_assertions)]
        {
            use std::{thread, time::Duration};

            println!("Dev mode: Sleeping for 2 second...");
            thread::sleep(Duration::from_secs(2));
        }

        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        // 1. Prepare Script via Driver
        let (script_content, key_gen) = self.bull_driver.remove();
        let script = Script::new(script_content);

        // Pre-load the script
        script.load_async(&mut *redis_conn).await?;

        let prefix = format!(
            "{}:{}:",
            self.connection_pool.metadata.bullmq_prefix, queue_name
        );

        let remove_children_val = if remove_children { "1" } else { "0" };
        let mut all_results = Vec::with_capacity(job_ids.len());

        // 2. Chunk the deletions (200 is better here as deletion triggers more internal logic)
        for chunk in job_ids.chunks(200) {
            let mut pipe = redis::pipe();

            for job_id in chunk {
                // Generate the keys required for removal
                // Standard BullMQ remove script usually requires 2 keys: [jobKey, repeatKey]
                let keys = key_gen(self.prefix, &queue_name, job_id);
                let mut invocation = script.prepare_invoke();

                for key in &keys {
                    invocation.key(key);
                }

                // Define ARGV based on the BullMQ remove script header
                invocation
                    .arg(job_id) // ARGV[1] jobId
                    .arg(remove_children_val) // ARGV[2] remove children flag
                    .arg(&prefix); // ARGV[3] prefix

                pipe.invoke_script(&invocation);
            }

            // 3. Execute and map
            // Results: 1 = Successfully removed, 0 = Job not found
            let chunk_results: Vec<i64> = pipe.query_async(&mut *redis_conn).await?;
            all_results.extend(chunk_results.iter().map(|r| r.to_string()));
        }

        Ok(all_results)
    }

    pub async fn delete_all_jobs_in_state_service(
        &self,
        queue_name: String,
        state: JobStatus, // "failed", "completed", "delayed", "wait"
        remove_children: bool,
    ) -> Result<String> {
        let mut redis_conn = self
            .connection_pool
            .pool
            .get()
            .await
            .context("Connection Failed!")?;

        let raw_prefix = self.prefix.trim_end_matches(':');
        let prefix = format!("{}:{}:", raw_prefix, queue_name);

        // Determine the key for the state
        let suffix = state.redis_key_suffix();

        let target_key = format!("{}{}", prefix, suffix);

        let chunk_size = 1000;
        let mut total_deleted = 0;

        loop {
            // 1. Fetch IDs based on structure type
            // Wait/Active/Paused are LISTS (LRANGE). Failed/Completed/Delayed are ZSETS (ZRANGE).
            let job_ids: Vec<String> = match state {
                JobStatus::Wait | JobStatus::Paused | JobStatus::Active => {
                    redis_conn.lrange(&target_key, 0, chunk_size - 1).await?
                }
                _ => redis_conn.zrange(&target_key, 0, chunk_size - 1).await?,
            };

            if job_ids.is_empty() {
                break;
            }

            // 2. Call the bulk delete logic
            // For now, we call the service we just wrote (which works, but less efficient to reconnect repeatedly)
            let _ = self
                .delete_jobs_service(queue_name.clone(), job_ids.clone(), remove_children)
                .await?;

            total_deleted += job_ids.len();
            // Safety Break: If we deleted 0 (shouldn't happen if list was not empty), break loop
            if job_ids.is_empty() {
                break;
            }
        }

        Ok(format!(
            "Successfully deleted {} jobs from {}",
            total_deleted,
            state.redis_key_suffix()
        ))
    }
}
