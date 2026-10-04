pub(crate) mod connection;
pub(crate) mod job;
pub(crate) mod queue;
pub(crate) mod user;
pub(crate) mod workspace;

use axum::{routing::get, Router};

use crate::{controller, server::AppState};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(controller::health))
        .merge(user::routes())
        .merge(connection::routes())
        .merge(workspace::routes())
        .merge(queue::routes())
        .merge(job::routes())
        .with_state(state)
}
