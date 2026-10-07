use crate::controller;
use axum::{
    Router,
    routing::{get, post},
};
use bullastrator_core::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/tabs",
            get(controller::tab::list).post(controller::tab::create),
        )
        .route("/tabs/active", get(controller::tab::active))
        .route("/tabs/reorder", post(controller::tab::reorder))
        .route(
            "/tabs/{tab_id}",
            get(controller::tab::get)
                .patch(controller::tab::update)
                .delete(controller::tab::delete),
        )
        .route("/tabs/{tab_id}/select", post(controller::tab::set_active))
}
