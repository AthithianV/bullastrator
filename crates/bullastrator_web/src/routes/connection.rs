use crate::controller;
use axum::{
    Router,
    routing::{get, post},
};
use bullastrator_core::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/connections",
            get(controller::connection::list).post(controller::connection::create),
        )
        .route(
            "/connections/{connection_id}",
            get(controller::connection::get)
                .patch(controller::connection::update)
                .delete(controller::connection::delete),
        )
        .route(
            "/connections/redis/test",
            post(controller::connection::test_redis),
        )
        .route(
            "/connections/{connection_id}/redis/health",
            get(controller::connection::redis_health),
        )
        .route(
            "/connections/redis/version",
            post(controller::connection::redis_version),
        )
}
