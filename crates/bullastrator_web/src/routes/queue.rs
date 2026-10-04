use axum::{routing::{get, post}, Router};
use crate::{controller, server::AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/queues", get(controller::queue::all_queues))
        .route("/queues/{queue_name}", get(controller::queue::queue_details))
        .route("/queues/{queue_name}/pause", post(controller::queue::pause_queue))
}
