use crate::controller;
use axum::{
    Router,
    routing::{get, post},
};
use bullastrator_core::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(controller::user::register))
        .route("/auth/login", post(controller::user::login))
        .route("/auth/logout", post(controller::user::logout))
}

pub fn global_admin_user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(controller::user::list_users))
        .route(
            "/roles",
            get(controller::user::list_roles).post(controller::user::create_role),
        )
        .route("/roles/options", get(controller::user::role_options))
        .route(
            "/roles/{role_id}",
            get(controller::user::get_role)
                .patch(controller::user::update_role)
                .delete(controller::user::delete_role),
        )
}
