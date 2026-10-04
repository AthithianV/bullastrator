use crate::{controller, server::AppState};
use axum::{routing::post, Router};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(controller::user::register))
        .route("/auth/login", post(controller::user::login))
        .route("/auth/logout", post(controller::user::logout))
}
