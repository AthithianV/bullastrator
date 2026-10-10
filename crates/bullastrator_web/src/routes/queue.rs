use crate::controller;
use axum::{
    Router,
    routing::{get, post},
};
use bullastrator_core::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/queues/{connection_id}",
            get(controller::queue::all_queues),
        )
        .route(
            "/queues/{connection_id}/{queue_name}",
            get(controller::queue::queue_details),
        )
        .route(
            "/queues/{connection_id}/{queue_name}/pause",
            post(controller::queue::pause_queue),
        )
}
