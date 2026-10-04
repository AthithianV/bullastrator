pub(crate) mod connection;
pub(crate) mod job;
pub(crate) mod queue;
pub(crate) mod user;
pub(crate) mod workspace;

use axum::{
    Router,
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use axum_extra::extract::CookieJar;
use bullastrator_core::state::AppState;

use crate::{controller, models::user::AuthenticatedUser};

pub async fn auth_middleware(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let session_token = jar
        .get("session")
        .map(|cookie| cookie.value().to_owned())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user_id = state
        .users
        .get_user_id(&session_token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    request
        .extensions_mut()
        .insert(AuthenticatedUser { id: user_id });

    Ok(next.run(request).await)
}

pub fn router(state: AppState) -> Router {
    let public_routes = Router::new()
        .route("/health", get(controller::health))
        .merge(user::routes());

    let protected_routes = Router::new()
        .merge(connection::routes())
        .merge(workspace::routes())
        .merge(queue::routes())
        .merge(job::routes())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    public_routes.merge(protected_routes).with_state(state)
}
