use std::time::{SystemTime, UNIX_EPOCH};

use rmp_serde::Serializer;
use serde::Serialize;
use serde_json::Value;

use crate::bull_drivers::driver_registry::BullmqDriver;

use crate::model::job_model::{JobOptions, JobStatus};
use crate::scripts::script_path::v5::{
    ADD_DELAYED_JOB_SCRIPT, ADD_JOB_SCRIPT, ADD_PRIORITIZED_JOB_SCRIPT, CLEAN_QUEUE_SCRIPT,
    GET_ALL_QUEUES, GET_JOB_COUNT_SCRIPT, GET_JOB_DATA_SCRIPT, GET_JOB_LOGS, GET_JOB_SCRIPT,
    PAUSE_QUEUE_SCRIPT, PROMOTE_JOB_SCRIPT, REMOVE_JOB_SCRIPT, REPROCESS_JOB_SCRIPT,
    SEARCH_JOB_SCRIPT, UPDATE_JOB_DATA_SCRIPT,
};

pub struct BullmqV5Driver;

impl BullmqDriver for BullmqV5Driver {
    fn version(&self) -> &str {
        "5.x"
    }

    fn get_queues(&self) -> &'static str {
        GET_ALL_QUEUES
    }

    fn add_job_args(
        &self,
        prefix: &str,
        queue_name: &str,
        job_name: &str,
        data: &Value,
        opts: &JobOptions,
    ) -> Vec<Vec<u8>> {
        let args_array = (
            format!("{}:{}:", prefix, queue_name),
            String::new(),
            job_name.to_string(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
            serde_json::Value::Null,
            serde_json::Value::Null,
            serde_json::Value::Null,
            serde_json::Value::Null,
            serde_json::Value::Null,
        );

        let mut args_packed = Vec::new();
        args_array
            .serialize(&mut Serializer::new(&mut args_packed))
            .unwrap();

        let data_json = serde_json::to_string(data).unwrap_or_else(|_| "{}".to_string());

        let mut opts_packed = Vec::new();
        opts.serialize(&mut Serializer::new(&mut opts_packed))
            .unwrap();

        vec![args_packed, data_json.into_bytes(), opts_packed]
    }

    fn add(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str) -> Vec<String> + Send + Sync>,
    ) {
        (
            ADD_JOB_SCRIPT,
            Box::new(|prefix, queue_name| {
                let p = format!("{}:{}:", prefix, queue_name);
                vec![
                    format!("{}wait", p),      // KEYS[1]
                    format!("{}paused", p),    // KEYS[2]
                    format!("{}meta", p),      // KEYS[3]
                    format!("{}id", p),        // KEYS[4]
                    format!("{}completed", p), // KEYS[5]
                    format!("{}delayed", p),   // KEYS[6]
                    format!("{}active", p),    // KEYS[7]
                    format!("{}events", p),    // KEYS[8]
                    format!("{}marker", p),    // KEYS[9]
                ]
            }),
        )
    }

    fn add_delayed(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str) -> Vec<String> + Send + Sync>,
    ) {
        (
            ADD_DELAYED_JOB_SCRIPT,
            Box::new(|prefix, queue_name| {
                let p = format!("{}:{}:", prefix, queue_name);
                vec![
                    format!("{}marker", p),    // KEYS[1]
                    format!("{}meta", p),      // KEYS[2]
                    format!("{}id", p),        // KEYS[3]
                    format!("{}delayed", p),   // KEYS[4]
                    format!("{}completed", p), // KEYS[5]
                    format!("{}events", p),    // KEYS[6]
                ]
            }),
        )
    }

    fn add_prioritized(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str) -> Vec<String> + Send + Sync>,
    ) {
        (
            ADD_PRIORITIZED_JOB_SCRIPT,
            Box::new(|prefix, queue_name| {
                let p = format!("{}:{}:", prefix, queue_name);
                vec![
                    format!("{}marker", p),      // KEYS[1]
                    format!("{}meta", p),        // KEYS[3]
                    format!("{}id", p),          // KEYS[4]
                    format!("{}prioritized", p), // KEYS[5]
                    format!("{}delayed", p),     // KEYS[6]
                    format!("{}completed", p),   // KEYS[7]
                    format!("{}active", p),      // KEYS[7]
                    format!("{}events", p),      // KEYS[8]
                    format!("{}pc", p),          // KEYS[9]
                ]
            }),
        )
    }

    fn remove(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> Vec<String> + Send + Sync>,
    ) {
        (
            REMOVE_JOB_SCRIPT,
            Box::new(|prefix, queue_name, job_id| {
                let base = format!("{}:{}:", prefix, queue_name);
                vec![format!("{}{}", base, job_id), format!("{}repeat", base)]
            }),
        )
    }

    fn get_counts(&self) -> &'static str {
        GET_JOB_COUNT_SCRIPT
    }

    fn search(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, JobStatus) -> Vec<String> + Send + Sync>,
    ) {
        (
            SEARCH_JOB_SCRIPT,
            Box::new(|prefix, name, status| {
                let p = format!("{}:{}:{}", prefix, name, status.redis_key_suffix());
                vec![p]
            }),
        )
    }

    fn retry(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> Vec<String> + Send + Sync>,
        Box<dyn Fn(&str, bool) -> Vec<String> + Send + Sync>, // Args generator
    ) {
        (
            REPROCESS_JOB_SCRIPT,
            Box::new(|prefix, queue_name, job_id| {
                let p = format!("{}:{}:", prefix, queue_name);
                vec![
                    format!("{}{}", p, job_id), // Job key prefix
                    format!("{}events", p),     // KEYS[2]
                    format!("{}failed", p),     // KEYS[3]
                    format!("{}wait", p),       // KEYS[4]
                    format!("{}meta", p),       // KEYS[5]
                    format!("{}paused", p),     // KEYS[6]
                    format!("{}active", p),     // KEYS[7]
                    format!("{}marker", p),     // KEYS[8]
                ]
            }),
            Box::new(|job_id, to_front| {
                vec![
                    job_id.to_string(), // ARGV[1] Job ID
                    if to_front {
                        // ARGV[2] LPUSH for LIFO, RPUSH for FIFO
                        "LPUSH".to_string()
                    } else {
                        "RPUSH".to_string()
                    },
                    "failedReason".to_string(), // ARGV[3]
                    "failed".to_string(),       // ARGV[4]
                    "0".to_string(),            // ARGV[5]
                    "0".to_string(),            // ARGV[6]
                ]
            }),
        )
    }

    fn reprocess(
        &self,
    ) -> (
        &'static str,                                               // Script to reprocess a job
        Box<dyn Fn(&str, &str, &str) -> Vec<String> + Send + Sync>, // Key generator
        Box<dyn Fn(&str, bool) -> Vec<String> + Send + Sync>,       // Args generator
    ) {
        (
            REPROCESS_JOB_SCRIPT,
            Box::new(|prefix, queue_name, job_id| {
                let p = format!("{}:{}:", prefix, queue_name);
                vec![
                    format!("{}{}", p, job_id), // KEYS[1] Job key prefix
                    format!("{}events", p),     // KEYS[2]
                    format!("{}completed", p),  // KEYS[3]
                    format!("{}wait", p),       // KEYS[4]
                    format!("{}meta", p),       // KEYS[5]
                    format!("{}paused", p),     // KEYS[6]
                    format!("{}active", p),     // KEYS[7]
                    format!("{}marker", p),     // KEYS[8]
                ]
            }),
            Box::new(|job_id, to_front| {
                vec![
                    job_id.to_string(), // ARGV[1] Job ID
                    if to_front {
                        // ARGV[2] LPUSH for LIFO, RPUSH for FIFO
                        "LPUSH".to_string()
                    } else {
                        "RPUSH".to_string()
                    },
                    "returnvalue".to_string(), // ARGV[3]
                    "completed".to_string(),   // ARGV[4]
                    "0".to_string(),           // ARGV[5]
                    "0".to_string(),           // ARGV[6]
                ]
            }),
        )
    }

    fn promote(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str) -> Vec<String> + Send + Sync>,
    ) {
        (
            PROMOTE_JOB_SCRIPT,
            Box::new(|prefix, queue_name| {
                let p = format!("{}:{}:", prefix, queue_name);
                vec![
                    format!("{}delayed", p),     // KEYS[1]
                    format!("{}wait", p),        // KEYS[2]
                    format!("{}paused", p),      // KEYS[3]
                    format!("{}meta", p),        // KEYS[4]
                    format!("{}prioritized", p), // KEYS[5]
                    format!("{}active", p),      // KEYS[6]
                    format!("{}pc", p),          // KEYS[7] Priority Counter
                    format!("{}events", p),      // KEYS[8] Event Stream
                    format!("{}marker", p),      // KEYS[9]
                ]
            }),
        )
    }

    fn get(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> String + Send + Sync>,
    ) {
        (
            GET_JOB_SCRIPT,
            Box::new(|prefix, queue_name, status_suffix| {
                format!("{}:{}:{}", prefix, queue_name, status_suffix) // KEYS[1]
            }),
        )
    }

    fn get_job_data(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> String + Send + Sync>,
    ) {
        (
            GET_JOB_DATA_SCRIPT,
            Box::new(|prefix, queue_name, status| format!("{}:{}:{}", prefix, queue_name, status)),
        )
    }

    fn get_job_logs(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> Vec<String> + Send + Sync>,
    ) {
        (
            GET_JOB_LOGS,
            Box::new(|prefix, queue_name, job_id| {
                vec![
                    format!("{}:{}:", prefix, queue_name), // KEYS[1]
                    job_id.to_string(),                    // KEYS[2]
                ]
            }),
        )
    }

    fn update(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> String + Send + Sync>,
    ) {
        (
            UPDATE_JOB_DATA_SCRIPT,
            Box::new(|prefix, queue_name, job_id| {
                format!("{}:{}:{}", prefix, queue_name, job_id) // KEYS[1]
            }),
        )
    }

    fn pause(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, bool) -> Vec<String> + Send + Sync>,
    ) {
        (
            PAUSE_QUEUE_SCRIPT,
            Box::new(|prefix, queue_name, should_pause| {
                let base = format!("{}:{}:", prefix, queue_name);
                let wait_key = format!("{}wait", base);
                let paused_key = format!("{}paused", base);

                let (src, dest) = if should_pause {
                    (wait_key, paused_key)
                } else {
                    (paused_key, wait_key)
                };

                vec![
                    src,
                    dest,
                    format!("{}meta", base),
                    format!("{}prioritized", base),
                    format!("{}events", base),
                    format!("{}delayed", base),
                    format!("{}marker", base),
                ]
            }),
        )
    }

    fn clean(
        &self,
    ) -> (
        &'static str,
        Box<dyn Fn(&str, &str, &str) -> Vec<String> + Send + Sync>,
    ) {
        (
            CLEAN_QUEUE_SCRIPT,
            Box::new(|prefix, queue_name, set_name| {
                // In BullMQ, keys are typically {prefix}:{queue_name}:{set_name}
                let base = format!("{}:{}:", prefix, queue_name);

                vec![
                    format!("{}{}", base, set_name), // KEYS[1]: The specific set (e.g., 'completed')
                    format!("{}events", base),       // KEYS[2]: Events stream for 'cleaned' event
                    format!("{}repeat", base),       // KEYS[3]: Repeatable jobs metadata
                ]
            }),
        )
    }
}
