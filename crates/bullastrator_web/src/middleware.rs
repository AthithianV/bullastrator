use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use bullastrator_core::state::AppState;

use crate::models::{user::AuthenticatedUser, workspace::RequiredWorkspaceRole};

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

pub async fn role_middleware(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<AuthenticatedUser>,
    axum::Extension(required_role): axum::Extension<RequiredWorkspaceRole>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let workspace_id = state
        .workspaces
        .active_id(&user.id)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    state
        .workspaces
        .check_permission(&workspace_id, &user.id, required_role.workspace_role)
        .await
        .map_err(|_| StatusCode::FORBIDDEN)?;

    Ok(next.run(request).await)
}
