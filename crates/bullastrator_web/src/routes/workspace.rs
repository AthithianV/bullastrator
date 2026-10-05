use crate::controller;
use axum::{
    Router,
    routing::{get, post},
};
use bullastrator_core::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/workspaces", post(controller::workspace::create))
        .route("/workspaces/active", get(controller::workspace::active))
        .route(
            "/workspaces/{workspace_id}",
            get(controller::workspace::get)
                .patch(controller::workspace::update)
                .delete(controller::workspace::delete),
        )
        .route(
            "/workspaces/{workspace_id}/select",
            post(controller::workspace::select),
        )
        .route(
            "/users/{user_id}/workspaces",
            get(controller::workspace::list_for_user),
        )
        .route(
            "/connections/{connection_id}/members",
            get(controller::workspace::list_members).post(controller::workspace::add_member),
        )
        .route(
            "/connections/{connection_id}/members/{user_id}",
            axum::routing::patch(controller::workspace::update_member)
                .delete(controller::workspace::remove_member),
        )
}
