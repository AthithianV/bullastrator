use axum::{routing::{get, post}, Router};
use crate::{controller, server::AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/connections/redis/test", post(controller::connection::test_redis))
        .route("/connections/redis/health", get(controller::connection::redis_health))
        .route("/connections/redis/version", post(controller::connection::redis_version))
}
