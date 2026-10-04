use crate::controller;
use axum::{
    Router,
    routing::{get, post},
};
use bullastrator_core::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/connections/{connection_id}/queues",
            get(controller::queue::all_queues),
        )
        .route(
            "/connections/{connection_id}/queues/{queue_name}",
            get(controller::queue::queue_details),
        )
        .route(
            "/connections/{connection_id}/queues/{queue_name}/pause",
            post(controller::queue::pause_queue),
        )
}
