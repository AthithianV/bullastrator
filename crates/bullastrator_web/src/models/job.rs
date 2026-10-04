use bullastrator_core::models::job::{JobFilterType, JobStatus, RetryJobStatus, RetryStrategy};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JobListQuery {
    pub status: JobStatus,
    pub cursor: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JobStatusQuery {
    pub status: JobStatus,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchRequest {
    pub status: JobStatus,
    pub filters: Vec<JobFilterType>,
    pub cursor: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize)]
pub(crate) struct JobDataRequest {
    pub job_ids: Vec<String>,
    pub status: JobStatus,
    pub start: Option<i32>,
    pub end: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateJobRequest {
    pub job_id: String,
    pub data: serde_json::Value,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JobCountsRequest {
    pub queue_names: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RetryJobsRequest {
    pub job_ids: Vec<String>,
    pub strategy: RetryStrategy,
    pub status: RetryJobStatus,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JobIdsRequest {
    pub job_ids: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeleteJobsRequest {
    pub job_ids: Vec<String>,
    pub remove_children: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RetryAllRequest {
    pub strategy: RetryStrategy,
    pub status: RetryJobStatus,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeleteStateRequest {
    pub state: JobStatus,
    pub remove_children: bool,
}
