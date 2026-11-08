use crate::controller;
use axum::{Router, routing::{get, post}};
use bullastrator_core::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(controller::user::register))
        .route("/auth/login", post(controller::user::login))
        .route("/auth/logout", post(controller::user::logout))
}

pub fn protected_routes() -> Router<AppState> {
    Router::new().route("/users", get(controller::user::list_users))
}
