pub(crate) mod connection;
pub(crate) mod job;
pub(crate) mod queue;
pub(crate) mod tab;
pub(crate) mod user;
pub(crate) mod workspace;

use axum::{
    Router,
    middleware::{self},
    routing::get,
};
use bullastrator_core::state::AppState;

use crate::{controller, middleware::auth_middleware};

pub fn router(state: AppState) -> Router {
    let public_routes = Router::new()
        .route("/health", get(controller::health))
        .merge(user::routes());

    let resource_routes = Router::new()
        .merge(connection::routes())
        .merge(queue::routes())
        .merge(tab::routes())
        .merge(job::routes());

    let protected_routes = Router::new()
        .merge(workspace::routes())
        .merge(resource_routes)
        .route("/auth/session", get(controller::user::session))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    public_routes.merge(protected_routes).with_state(state)
}
