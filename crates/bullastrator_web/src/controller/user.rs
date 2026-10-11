use crate::{controller::ApiResult, models::user::AuthenticatedUser};
use axum::{Json, extract::State};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use bullastrator_core::{
    models::user::{AuthResponse, LoginRequest, RegisterRequest, TokenRequest},
    state::AppState,
};

pub(crate) async fn register(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<RegisterRequest>,
) -> ApiResult<(CookieJar, Json<AuthResponse>)> {
    let response = state.users.register(request).await?;
    Ok((jar.add(session_cookie(&response.token)), Json(response)))
}

pub(crate) async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> ApiResult<(CookieJar, Json<AuthResponse>)> {
    let response = state.users.login(request).await?;
    Ok((jar.add(session_cookie(&response.token)), Json(response)))
}

pub(crate) async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<TokenRequest>,
) -> ApiResult<(CookieJar, Json<serde_json::Value>)> {
    let revoked = state.users.logout(&request.token).await?;
    Ok((
        jar.remove(Cookie::build(("session", "")).path("/").build()),
        Json(serde_json::json!({"revoked": revoked})),
    ))
}

fn session_cookie(token: &str) -> Cookie<'static> {
    Cookie::build(("session", token.to_owned()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build()
}

pub(crate) async fn session(
    axum::extract::Extension(user): axum::Extension<AuthenticatedUser>,
) -> Json<AuthenticatedUser> {
    Json(user)
}
