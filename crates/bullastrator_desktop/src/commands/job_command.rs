use serde_json::Value;
use tauri::{State, command};
use tracing::error;
use uuid::Uuid;

use crate::model::job_model::{
    AddJobModel, JobDetails, JobFilterType, JobSearchResult, JobStatus, RetryJobStatus,
    RetryStrategy,
};
use crate::model::queue_model::QueueJobCounts;

use crate::service::job_service::JobService;

use crate::utilities::app_state::AppState;
use crate::utilities::redis_utils::get_or_create_redis_connection;

#[command]
pub async fn get_jobs_in_queue(
    app_state: State<'_, AppState>,
    connection_id: Uuid,
    queue_name: String,
    status: JobStatus,
    filters: Vec<JobFilterType>,
    cursor: usize,
    limit: usize,
) -> Result<JobSearchResult, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred while connecting: {:?}", e);
            e.to_string()
        })?;

    let job_service = JobService::new(&connection_pool);

    if filters.is_empty() {
        let jobs = job_service
            .get_jobs_in_queue_service(queue_name, status, cursor, limit)
            .await
            .map_err(|e| {
                error!("An error occurred while getting all jobs in queue: {:?}", e);
                e.to_string()
            })?;

        Ok(jobs)
    } else {
        let jobs = job_service
            .search_jobs_in_queue_service(queue_name, status, filters, cursor, limit)
            .await
            .map_err(|e| {
                error!("An error occurred while searching: {:?}", e);
                e.to_string()
            })?;

        Ok(jobs)
    }
}

#[command]
pub async fn get_jobs_by_id(
    app_state: State<'_, AppState>,
    connection_id: Uuid,
    queue_name: String,
    job_id: String,
    status: JobStatus,
) -> Result<Option<JobDetails>, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    let job_service = JobService::new(&connection_pool);

    let job = job_service
        .get_jobs_by_id_service(queue_name, job_id, status)
        .await
        .map_err(|e| {
            error!("An error occurred while getting job by count: {:?}", e);
            e.to_string()
        })?;

    Ok(job)
}

#[command]
pub async fn get_jobs_data(
    app_state: State<'_, AppState>,
    connection_id: Uuid,
    queue_name: String,
    job_ids: Vec<String>,
    status: JobStatus,
    start: Option<i32>,
    end: Option<i32>,
) -> Result<Vec<Value>, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    let job_service = JobService::new(&connection_pool);

    let job = job_service
        .get_job_data(queue_name, job_ids, status, start, end)
        .await
        .map_err(|e| {
            error!("An error occurred while getting job by count: {:?}", e);
            e.to_string()
        })?;

    Ok(job)
}

#[command]
pub async fn get_job_logs(
    app_state: State<'_, AppState>,
    connection_id: Uuid,
    queue_name: String,
    job_id: String,
) -> Result<Vec<String>, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    let job_service = JobService::new(&connection_pool);

    let job = job_service
        .get_job_logs_service(queue_name, job_id)
        .await
        .map_err(|e| {
            error!("An error occurred while getting job by count: {:?}", e);
            e.to_string()
        })?;

    Ok(job)
}

#[command]
pub async fn get_job_count_in_queue(
    app_state: State<'_, AppState>,
    connection_id: Uuid,
    queue_name: String,
) -> Result<QueueJobCounts, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    let job_service = JobService::new(&connection_pool);
    let mut job_counts = job_service
        .get_job_count_in_queue_service(vec![&queue_name])
        .await
        .map_err(|e| {
            error!("An error occurred while getting count in queue: {:?}", e);
            e.to_string()
        })?;

    Ok(job_counts.remove(0))
}

#[command]
pub async fn add_job_to_queue(
    app_state: State<'_, AppState>,
    connection_id: Uuid,
    queue_name: String,
    jobs: Vec<AddJobModel>,
) -> Result<Vec<String>, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    let job_service = JobService::new(&connection_pool);
    let job_counts = job_service
        .add_job_to_queue_service(queue_name, jobs)
        .await
        .map_err(|e| {
            error!("An error occurred while adding job to queue: {:?}", e);
            e.to_string()
        })?;

    Ok(job_counts)
}

#[command]
pub async fn update_job_data(
    app_state: State<'_, AppState>,
    queue_name: String,
    connection_id: Uuid,
    job_id: String,
    job_data: Value,
) -> Result<String, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    let job_service = JobService::new(&connection_pool);
    let result: String = job_service
        .update_job_data_service(queue_name, job_id, job_data)
        .await
        .map_err(|e| {
            error!("An error occurred while updating jobs: {:?}", e);
            e.to_string()
        })?;

    Ok(result)
}

#[command]
pub async fn retry_failed_jobs(
    app_state: State<'_, AppState>,
    queue_name: String,
    connection_id: Uuid,
    job_ids: Vec<String>,
    strategy: RetryStrategy,
    retry_job_status: RetryJobStatus,
) -> Result<Vec<String>, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    let job_service = JobService::new(&connection_pool);
    let result = job_service
        .retry_failed_jobs_service(queue_name, job_ids, strategy, &retry_job_status)
        .await
        .map_err(|e| {
            error!("An error occurred while retrying failed jobs: {:?}", e);
            e.to_string()
        })?;

    Ok(result)
}

#[command]
pub async fn retry_all_failed_jobs(
    app_state: State<'_, AppState>,
    queue_name: String,
    connection_id: Uuid,
    strategy: RetryStrategy,
    retry_job_status: RetryJobStatus,
) -> Result<String, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    let job_service = JobService::new(&connection_pool);
    let result = job_service
        .retry_all_failed_jobs_service(queue_name, strategy, &retry_job_status)
        .await
        .map_err(|e| {
            error!("An error occurred while retrying all jobs: {:?}", e);
            e.to_string()
        })?;

    Ok(result)
}

#[command]
pub async fn promote_jobs(
    app_state: State<'_, AppState>,
    queue_name: String,
    connection_id: Uuid,
    job_ids: Vec<String>,
) -> Result<Vec<String>, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    let job_service = JobService::new(&connection_pool);
    let result = job_service
        .promote_jobs_service(queue_name, job_ids)
        .await
        .map_err(|e| {
            error!("An error occurred while promoting jobs: {:?}", e);
            e.to_string()
        })?;

    Ok(result)
}

#[command]
pub async fn promote_all_jobs(
    app_state: State<'_, AppState>,
    queue_name: String,
    connection_id: Uuid,
) -> Result<String, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    let job_service = JobService::new(&connection_pool);
    let result = job_service
        .promote_all_jobs_service(queue_name)
        .await
        .map_err(|e| {
            error!("An error occurred while promoting all jobs: {:?}", e);
            e.to_string()
        })?;

    Ok(result)
}

#[command]
pub async fn delete_jobs(
    app_state: State<'_, AppState>,
    queue_name: String,
    connection_id: Uuid,
    job_ids: Vec<String>,
) -> Result<Vec<String>, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    let job_service = JobService::new(&connection_pool);
    let result = job_service
        .delete_jobs_service(queue_name, job_ids, false)
        .await
        .map_err(|e| {
            error!("An error occurred while deleting jobs: {:?}", e);
            e.to_string()
        })?;

    Ok(result)
}

#[command]
pub async fn delete_all_jobs(
    app_state: State<'_, AppState>,
    queue_name: String,
    status: JobStatus,
    connection_id: Uuid,
) -> Result<String, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    let job_service = JobService::new(&connection_pool);
    let result = job_service
        .delete_all_jobs_in_state_service(queue_name, status, false)
        .await
        .map_err(|e| {
            error!("An error occurred while deleting all jobs: {:?}", e);
            e.to_string()
        })?;

    Ok(result)
}
