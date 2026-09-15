use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::utilities::redis_utils::{parse_u32, parse_u64};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    // Identity
    pub id: String,
    pub name: String,

    // Status & Progress
    #[serde(default)]
    pub attempts_made: u32,

    // Timestamps (stored as milliseconds in BullMQ)
    pub timestamp: u64,
    #[serde(default)]
    pub delay: u64,
    #[serde(default)]
    pub finished_on: Option<u64>,
    #[serde(default)]
    pub processed_on: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobDetails {
    // Identity
    pub id: String,
    pub name: String,

    // Payloads
    pub data: Value,
    pub return_value: Value,
    pub opts: JobOptions,

    // Status & Progress
    #[serde(default)]
    pub progress: Value,
    pub attempts_made: u32,

    // Timestamps (stored as milliseconds in BullMQ)
    pub timestamp: u64,
    #[serde(default)]
    pub delay: u64,
    #[serde(default)]
    pub finished_on: Option<u64>,
    #[serde(default)]
    pub processed_on: Option<u64>,

    // Results & Errors
    #[serde(default)]
    pub returnvalue: Option<Value>,
    #[serde(default)]
    pub failed_reason: Option<String>,
    #[serde(default)]
    pub stacktrace: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobDataResponse {
    pub count: usize,
    pub jobs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobOptions {
    #[serde(default)]
    pub priority: Option<u32>,
    #[serde(default)]
    pub delay: Option<u64>,
    #[serde(default)]
    pub attempts: Option<u32>,

    // Backoff can be a simple number or an object in BullMQ
    #[serde(default)]
    pub backoff: Option<BackoffStrategy>,

    #[serde(default)]
    pub timeout: Option<u64>,
    #[serde(default)]
    pub remove_on_complete: Option<KeepJobs>, // boolean or integer or object
    #[serde(default)]
    pub remove_on_fail: Option<KeepJobs>, // boolean or integer or object
    #[serde(default)]
    pub stack_trace_limit: Option<u32>,
}

impl JobDetails {
    pub fn from_redis_map(id: String, map: &HashMap<String, String>) -> Self {
        let parse_json = |key: &str| -> Value {
            map.get(key)
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or(Value::Null)
        };

        // Numbers
        let parse_u64 =
            |key: &str| -> u64 { map.get(key).and_then(|s| s.parse().ok()).unwrap_or(0) };

        let parse_u32 =
            |key: &str| -> u32 { map.get(key).and_then(|s| s.parse().ok()).unwrap_or(0) };

        // Options
        let opts_raw = map.get("opts").map(|s| s.as_str()).unwrap_or("{}");
        let opts: JobOptions = serde_json::from_str(opts_raw).unwrap_or_else(|_| JobOptions {
            priority: None,
            delay: None,
            attempts: None,
            backoff: None,
            timeout: None,
            remove_on_complete: None,
            remove_on_fail: None,
            stack_trace_limit: None,
        });

        // Stack Trace
        let stacktrace: Vec<String> = map
            .get("stacktrace")
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();

        // Job data
        JobDetails {
            id,
            name: map
                .get("name")
                .cloned()
                .unwrap_or_else(|| "unknown".to_string()),

            // JSON Fields
            data: parse_json("data"),
            return_value: parse_json("returnvalue"),
            opts,
            progress: parse_json("progress"), // Progress can be number or object
            returnvalue: map
                .get("returnvalue")
                .and_then(|s| serde_json::from_str(s).ok()),

            // Numeric Fields
            attempts_made: parse_u32("attemptsMade"), // Note camelCase key in Redis usually
            timestamp: parse_u64("timestamp"),
            delay: parse_u64("delay"),
            finished_on: map.get("finishedOn").and_then(|s| s.parse().ok()),
            processed_on: map.get("processedOn").and_then(|s| s.parse().ok()),

            // Strings
            failed_reason: map.get("failedReason").cloned(),
            stacktrace,
        }
    }
}

impl Job {
    pub fn from_hmget_values(id: String, raw_data: &Vec<Option<String>>) -> Self {
        // Follow HMGET order:
        // 0: name
        // 1: timestamp
        // 2: processedOn
        // 3: delay
        // 4: priority
        // 5: attemptsMade
        // 6: finishedOn

        Job {
            id,
            // Use unwrap_or_else for strings to avoid allocation if not needed
            name: raw_data[0].clone().unwrap_or_else(|| "unknown".to_string()),

            // Parse timestamps (u64 is standard for JS timestamps)
            timestamp: parse_u64(&raw_data[1]),

            // Parse counters/settings
            delay: parse_u64(&raw_data[3]),
            attempts_made: parse_u32(&raw_data[5]),

            processed_on: raw_data[2].as_ref().and_then(|s| s.parse().ok()),
            finished_on: raw_data[6].as_ref().and_then(|s| s.parse().ok()),
        }
    }

    pub fn from_map(id: String, map: &HashMap<String, String>) -> Self {
        let parse_u64 =
            |key: &str| -> u64 { map.get(key).and_then(|s| s.parse().ok()).unwrap_or(0) };
        let parse_u32 =
            |key: &str| -> u32 { map.get(key).and_then(|s| s.parse().ok()).unwrap_or(0) };

        Job {
            id,
            name: map
                .get("name")
                .cloned()
                .unwrap_or_else(|| "unknown".to_string()),

            // Standard numeric fields
            timestamp: parse_u64("timestamp"),
            delay: parse_u64("delay"),
            attempts_made: parse_u32("attemptsMade"),

            // Option fields
            processed_on: map.get("processedOn").and_then(|s| s.parse().ok()),
            finished_on: map.get("finishedOn").and_then(|s| s.parse().ok()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Wait,
    Active,
    Completed,
    Failed,
    Delayed,
    Paused,
    Prioritized,
}

impl JobStatus {
    pub fn redis_key_suffix(&self) -> &str {
        match self {
            JobStatus::Wait => "wait",
            JobStatus::Active => "active",
            JobStatus::Completed => "completed",
            JobStatus::Failed => "failed",
            JobStatus::Delayed => "delayed",
            JobStatus::Paused => "paused",
            JobStatus::Prioritized => "prioritized",
        }
    }
}

// BullMQ Backoff can be slightly complex (number or object)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)] // Allows matching different JSON types
pub enum BackoffStrategy {
    Fixed(u64),
    Settings {
        #[serde(rename = "type")]
        strategy_type: String,
        delay: u64,
    },
}

// BullMQ "removeOnComplete" / "removeOnFail" can be a bool or an object
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum KeepJobs {
    Bool(bool),
    Count(u32),
    Settings {
        age: Option<u32>,
        count: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RetryStrategy {
    ToBack,
    ToFront,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RetryJobStatus {
    Completed,
    Failed,
}

impl RetryJobStatus {
    pub fn redis_key_suffix(&self) -> &str {
        match self {
            RetryJobStatus::Completed => "completed",
            RetryJobStatus::Failed => "failed",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum JobFilterType {
    // --- Direct Lookups ---
    JobId(String), // Exact ID match (Fastest)

    // --- Content Search ---
    Keyword(String), // Search inside 'data' JSON string

    // --- Metadata Filters ---
    FailedReason(String), // Partial match
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JobSearchResult {
    pub jobs: Vec<Job>,
    pub next_cursor: usize,   // Where to start the next request
    pub has_more: bool,       // True if we haven't reached the end of the queue
    pub scanned_count: usize, // For UI stats: "Scanned X items..."
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddJobModel {
    pub name: String,
    pub data: Value,
    pub opts: JobOptions,
}
