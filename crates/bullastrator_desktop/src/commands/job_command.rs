use bullastrator_core::{
    models::job::{
        AddJobModel, JobDetails, JobFilterType, JobSearchResult, JobStatus, QueueJobCounts,
        RetryJobStatus, RetryStrategy,
    },
    services::job::JobService,
    state::AppState,
};
use bullastrator_storage::models::WorkspaceRole;
use serde_json::Value;
use tauri::{State, command};

async fn service(
    state: &AppState,
    connection_id: &str,
    role: WorkspaceRole,
) -> Result<JobService, String> {
    let user = state
        .users
        .ensure_default_user()
        .await
        .map_err(|error| error.to_string())?;
    let redis = state
        .get_redis_connection(connection_id)
        .await
        .map_err(|error| error.to_string())?;
    JobService::for_user(&redis, &state.connections, &user.id, role)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn get_jobs_in_queue(
    state: State<'_, AppState>,
    connection_id: String,
    queue_name: String,
    status: JobStatus,
    filters: Vec<JobFilterType>,
    cursor: usize,
    limit: usize,
) -> Result<JobSearchResult, String> {
    let service = service(&state, &connection_id, WorkspaceRole::VIEWER).await?;
    service
        .get_jobs_in_queue_service(queue_name, status, filters, cursor, limit)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn get_jobs_by_id(
    state: State<'_, AppState>,
    connection_id: String,
    queue_name: String,
    job_id: String,
    status: JobStatus,
) -> Result<Option<JobDetails>, String> {
    let service = service(&state, &connection_id, WorkspaceRole::VIEWER).await?;
    service
        .get_jobs_by_id_service(queue_name, job_id, status)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn get_jobs_data(
    state: State<'_, AppState>,
    connection_id: String,
    queue_name: String,
    job_ids: Vec<String>,
    status: JobStatus,
    start: Option<i32>,
    end: Option<i32>,
) -> Result<Vec<Value>, String> {
    let service = service(&state, &connection_id, WorkspaceRole::VIEWER).await?;
    service
        .get_job_data(queue_name, job_ids, status, start, end)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn get_job_logs(
    state: State<'_, AppState>,
    connection_id: String,
    queue_name: String,
    job_id: String,
) -> Result<Vec<String>, String> {
    let service = service(&state, &connection_id, WorkspaceRole::VIEWER).await?;
    service
        .get_job_logs_service(queue_name, job_id)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn get_job_count_in_queue(
    state: State<'_, AppState>,
    connection_id: String,
    queue_name: String,
) -> Result<Vec<QueueJobCounts>, String> {
    let service = service(&state, &connection_id, WorkspaceRole::VIEWER).await?;
    let counts = service
        .get_job_count_in_queue_service(vec![&queue_name])
        .await
        .map_err(|error| error.to_string())?;
    Ok(counts)
}

#[command]
pub async fn add_job_to_queue(
    state: State<'_, AppState>,
    connection_id: String,
    queue_name: String,
    jobs: Vec<AddJobModel>,
) -> Result<Vec<String>, String> {
    let service = service(&state, &connection_id, WorkspaceRole::EDITOR).await?;
    service
        .add_job_to_queue_service(queue_name, jobs)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn update_job_data(
    state: State<'_, AppState>,
    queue_name: String,
    connection_id: String,
    job_id: String,
    job_data: Value,
) -> Result<String, String> {
    let service = service(&state, &connection_id, WorkspaceRole::EDITOR).await?;
    service
        .update_job_data_service(queue_name, job_id, job_data)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn retry_failed_jobs(
    state: State<'_, AppState>,
    queue_name: String,
    connection_id: String,
    job_ids: Vec<String>,
    strategy: RetryStrategy,
    retry_job_status: RetryJobStatus,
) -> Result<Vec<String>, String> {
    let service = service(&state, &connection_id, WorkspaceRole::EDITOR).await?;
    service
        .retry_failed_jobs_service(queue_name, job_ids, strategy, &retry_job_status)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn retry_all_failed_jobs(
    state: State<'_, AppState>,
    queue_name: String,
    connection_id: String,
    strategy: RetryStrategy,
    retry_job_status: RetryJobStatus,
) -> Result<String, String> {
    let service = service(&state, &connection_id, WorkspaceRole::EDITOR).await?;
    service
        .retry_all_failed_jobs_service(queue_name, strategy, &retry_job_status)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn promote_jobs(
    state: State<'_, AppState>,
    queue_name: String,
    connection_id: String,
    job_ids: Vec<String>,
) -> Result<Vec<String>, String> {
    let service = service(&state, &connection_id, WorkspaceRole::EDITOR).await?;
    service
        .promote_jobs_service(queue_name, job_ids)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn promote_all_jobs(
    state: State<'_, AppState>,
    queue_name: String,
    connection_id: String,
) -> Result<String, String> {
    let service = service(&state, &connection_id, WorkspaceRole::EDITOR).await?;
    service
        .promote_all_jobs_service(queue_name)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn delete_jobs(
    state: State<'_, AppState>,
    queue_name: String,
    connection_id: String,
    job_ids: Vec<String>,
) -> Result<Vec<String>, String> {
    let service = service(&state, &connection_id, WorkspaceRole::EDITOR).await?;
    service
        .delete_jobs_service(queue_name, job_ids, false)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn delete_all_jobs(
    state: State<'_, AppState>,
    queue_name: String,
    status: JobStatus,
    connection_id: String,
) -> Result<String, String> {
    let service = service(&state, &connection_id, WorkspaceRole::EDITOR).await?;
    service
        .delete_all_jobs_in_state_service(queue_name, status, false)
        .await
        .map_err(|error| error.to_string())
}
