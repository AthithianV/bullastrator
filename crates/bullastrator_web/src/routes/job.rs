use crate::controller;
use axum::{
    Router,
    routing::{get, post},
};
use bullastrator_core::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/connections/{connection_id}/queues/{queue_name}/jobs",
            get(controller::job::list_jobs).post(controller::job::add_jobs),
        )
        .route(
            "/connections/{connection_id}/queues/{queue_name}/jobs/search",
            post(controller::job::search_jobs),
        )
        .route(
            "/connections/{connection_id}/queues/{queue_name}/jobs/data",
            post(controller::job::job_data),
        )
        .route(
            "/connections/{connection_id}/queues/{queue_name}/jobs/counts",
            post(controller::job::job_counts),
        )
        .route(
            "/connections/{connection_id}/queues/{queue_name}/jobs/update",
            post(controller::job::update_job),
        )
        .route(
            "/connections/{connection_id}/queues/{queue_name}/jobs/retry",
            post(controller::job::retry_jobs),
        )
        .route(
            "/connections/{connection_id}/queues/{queue_name}/jobs/promote",
            post(controller::job::promote_jobs),
        )
        .route(
            "/connections/{connection_id}/queues/{queue_name}/jobs/delete",
            post(controller::job::delete_jobs),
        )
        .route(
            "/connections/{connection_id}/queues/{queue_name}/jobs/{job_id}",
            get(controller::job::get_job),
        )
        .route(
            "/connections/{connection_id}/queues/{queue_name}/jobs/{job_id}/logs",
            get(controller::job::job_logs),
        )
        .route(
            "/connections/{connection_id}/queues/{queue_name}/jobs/retry-all",
            post(controller::job::retry_all),
        )
        .route(
            "/connections/{connection_id}/queues/{queue_name}/jobs/promote-all",
            post(controller::job::promote_all),
        )
        .route(
            "/connections/{connection_id}/queues/{queue_name}/jobs/delete-state",
            post(controller::job::delete_state),
        )
}
