use crate::models::user::AuthenticatedUser;
use crate::{
    controller::ApiResult,
    models::job::{
        DeleteJobsRequest, DeleteStateRequest, JobCountsRequest, JobDataRequest, JobIdsRequest,
        JobListQuery, JobStatusQuery, RetryAllRequest, RetryJobsRequest, SearchRequest,
        UpdateJobRequest,
    },
};
use axum::{
    Extension, Json,
    extract::{Path, Query, State},
};
use bullastrator_core::{
    models::job::{AddJobModel, JobSearchResult},
    services::job::JobService,
    state::AppState,
};
use bullastrator_storage::models::WorkspaceRole;

pub(crate) async fn list_jobs(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue)): Path<(String, String)>,
    Query(query): Query<JobListQuery>,
) -> ApiResult<Json<JobSearchResult>> {
    let redis_connection = state.get_redis_connection(&connection_id).await?;
    let s = JobService::for_user(
        &redis_connection,
        &state.connections,
        &user.id,
        WorkspaceRole::VIEWER,
    )
    .await?;
    Ok(Json(
        s.get_jobs_in_queue_service(
            queue,
            query.status,
            query.cursor.unwrap_or(0),
            query.limit.unwrap_or(50),
        )
        .await?,
    ))
}

pub(crate) async fn get_job(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue, job_id)): Path<(String, String, String)>,
    Query(query): Query<JobStatusQuery>,
) -> ApiResult<Json<Option<bullastrator_core::models::job::JobDetails>>> {
    let redis_connection = state.get_redis_connection(&connection_id).await?;
    let s = JobService::for_user(
        &redis_connection,
        &state.connections,
        &user.id,
        WorkspaceRole::VIEWER,
    )
    .await?;
    Ok(Json(
        s.get_jobs_by_id_service(queue, job_id, query.status)
            .await?,
    ))
}

pub(crate) async fn job_logs(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue, job_id)): Path<(String, String, String)>,
) -> ApiResult<Json<Vec<String>>> {
    let redis_connection = state.get_redis_connection(&connection_id).await?;
    let s = JobService::for_user(
        &redis_connection,
        &state.connections,
        &user.id,
        WorkspaceRole::VIEWER,
    )
    .await?;
    Ok(Json(s.get_job_logs_service(queue, job_id).await?))
}

pub(crate) async fn add_jobs(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue)): Path<(String, String)>,
    Json(jobs): Json<Vec<AddJobModel>>,
) -> ApiResult<Json<Vec<String>>> {
    let redis_connection = state.get_redis_connection(&connection_id).await?;
    let s = JobService::for_user(
        &redis_connection,
        &state.connections,
        &user.id,
        WorkspaceRole::EDITOR,
    )
    .await?;
    Ok(Json(s.add_job_to_queue_service(queue, jobs).await?))
}

pub(crate) async fn search_jobs(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue)): Path<(String, String)>,
    Json(r): Json<SearchRequest>,
) -> ApiResult<Json<JobSearchResult>> {
    let redis_connection = state.get_redis_connection(&connection_id).await?;
    let s = JobService::for_user(
        &redis_connection,
        &state.connections,
        &user.id,
        WorkspaceRole::VIEWER,
    )
    .await?;
    Ok(Json(
        s.search_jobs_in_queue_service(
            queue,
            r.status,
            r.filters,
            r.cursor.unwrap_or(0),
            r.limit.unwrap_or(50),
        )
        .await?,
    ))
}

pub(crate) async fn job_data(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue)): Path<(String, String)>,
    Json(r): Json<JobDataRequest>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let redis_connection = state.get_redis_connection(&connection_id).await?;
    let s = JobService::for_user(
        &redis_connection,
        &state.connections,
        &user.id,
        WorkspaceRole::VIEWER,
    )
    .await?;
    Ok(Json(
        s.get_job_data(queue, r.job_ids, r.status, r.start, r.end)
            .await?,
    ))
}

pub(crate) async fn job_counts(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, _queue)): Path<(String, String)>,
    Json(r): Json<JobCountsRequest>,
) -> ApiResult<Json<Vec<bullastrator_core::models::job::QueueJobCounts>>> {
    let redis_connection = state.get_redis_connection(&connection_id).await?;
    let s = JobService::for_user(
        &redis_connection,
        &state.connections,
        &user.id,
        WorkspaceRole::VIEWER,
    )
    .await?;
    Ok(Json(
        s.get_job_count_in_queue_service(r.queue_names.iter().map(String::as_str).collect())
            .await?,
    ))
}

pub(crate) async fn update_job(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue)): Path<(String, String)>,
    Json(r): Json<UpdateJobRequest>,
) -> ApiResult<Json<String>> {
    let redis_connection = state.get_redis_connection(&connection_id).await?;
    let s = JobService::for_user(
        &redis_connection,
        &state.connections,
        &user.id,
        WorkspaceRole::EDITOR,
    )
    .await?;
    Ok(Json(
        s.update_job_data_service(queue, r.job_id, r.data).await?,
    ))
}

pub(crate) async fn retry_jobs(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue)): Path<(String, String)>,
    Json(r): Json<RetryJobsRequest>,
) -> ApiResult<Json<Vec<String>>> {
    let redis_connection = state.get_redis_connection(&connection_id).await?;
    let s = JobService::for_user(
        &redis_connection,
        &state.connections,
        &user.id,
        WorkspaceRole::EDITOR,
    )
    .await?;
    Ok(Json(
        s.retry_failed_jobs_service(queue, r.job_ids, r.strategy, &r.status)
            .await?,
    ))
}

pub(crate) async fn promote_jobs(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue)): Path<(String, String)>,
    Json(r): Json<JobIdsRequest>,
) -> ApiResult<Json<Vec<String>>> {
    let redis_connection = state.get_redis_connection(&connection_id).await?;
    let s = JobService::for_user(
        &redis_connection,
        &state.connections,
        &user.id,
        WorkspaceRole::EDITOR,
    )
    .await?;
    Ok(Json(s.promote_jobs_service(queue, r.job_ids).await?))
}

pub(crate) async fn delete_jobs(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue)): Path<(String, String)>,
    Json(r): Json<DeleteJobsRequest>,
) -> ApiResult<Json<Vec<String>>> {
    let redis_connection = state.get_redis_connection(&connection_id).await?;
    let s = JobService::for_user(
        &redis_connection,
        &state.connections,
        &user.id,
        WorkspaceRole::EDITOR,
    )
    .await?;
    Ok(Json(
        s.delete_jobs_service(queue, r.job_ids, r.remove_children)
            .await?,
    ))
}

pub(crate) async fn retry_all(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue)): Path<(String, String)>,
    Json(r): Json<RetryAllRequest>,
) -> ApiResult<Json<String>> {
    let redis_connection = state.get_redis_connection(&connection_id).await?;
    let s = JobService::for_user(
        &redis_connection,
        &state.connections,
        &user.id,
        WorkspaceRole::EDITOR,
    )
    .await?;
    Ok(Json(
        s.retry_all_failed_jobs_service(queue, r.strategy, &r.status)
            .await?,
    ))
}

pub(crate) async fn promote_all(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue)): Path<(String, String)>,
) -> ApiResult<Json<String>> {
    let redis_connection = state.get_redis_connection(&connection_id).await?;
    let s = JobService::for_user(
        &redis_connection,
        &state.connections,
        &user.id,
        WorkspaceRole::EDITOR,
    )
    .await?;
    Ok(Json(s.promote_all_jobs_service(queue).await?))
}

pub(crate) async fn delete_state(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue)): Path<(String, String)>,
    Json(r): Json<DeleteStateRequest>,
) -> ApiResult<Json<String>> {
    let redis_connection = state.get_redis_connection(&connection_id).await?;
    let s = JobService::for_user(
        &redis_connection,
        &state.connections,
        &user.id,
        WorkspaceRole::EDITOR,
    )
    .await?;
    Ok(Json(
        s.delete_all_jobs_in_state_service(queue, r.state, r.remove_children)
            .await?,
    ))
}
