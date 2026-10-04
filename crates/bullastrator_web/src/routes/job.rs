use axum::{routing::{get, post}, Router};
use crate::{controller, server::AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/queues/{queue_name}/jobs", get(controller::job::list_jobs).post(controller::job::add_jobs))
        .route("/queues/{queue_name}/jobs/search", post(controller::job::search_jobs))
        .route("/queues/{queue_name}/jobs/data", post(controller::job::job_data))
        .route("/queues/{queue_name}/jobs/counts", post(controller::job::job_counts))
        .route("/queues/{queue_name}/jobs/update", post(controller::job::update_job))
        .route("/queues/{queue_name}/jobs/retry", post(controller::job::retry_jobs))
        .route("/queues/{queue_name}/jobs/promote", post(controller::job::promote_jobs))
        .route("/queues/{queue_name}/jobs/delete", post(controller::job::delete_jobs))
        .route("/queues/{queue_name}/jobs/{job_id}", get(controller::job::get_job))
        .route("/queues/{queue_name}/jobs/{job_id}/logs", get(controller::job::job_logs))
        .route("/queues/{queue_name}/jobs/retry-all", post(controller::job::retry_all))
        .route("/queues/{queue_name}/jobs/promote-all", post(controller::job::promote_all))
        .route("/queues/{queue_name}/jobs/delete-state", post(controller::job::delete_state))
}
