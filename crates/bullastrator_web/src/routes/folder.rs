use crate::controller;
use axum::{Router, routing::{get, post}};
use bullastrator_core::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/folders", post(controller::folder::create))
        .route("/folders/connection/{connection_id}", get(controller::folder::list))
        .route("/folders/{folder_id}", get(controller::folder::get).patch(controller::folder::update).delete(controller::folder::delete))
        .route("/folders/{folder_id}/queues", get(controller::folder::queues))
        .route("/folders/{folder_id}/queues/reorder", post(controller::folder::reorder))
        .route("/folders/{folder_id}/queues/{queue_id}/toggle", post(controller::folder::toggle_queue))
}
