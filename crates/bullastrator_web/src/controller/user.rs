use crate::{controller::ApiResult, models::user::AuthenticatedUser};
use axum::{Json, extract::{Path, State}};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use bullastrator_core::{
    models::user::{AuthResponse, LoginRequest, RegisterRequest},
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
) -> ApiResult<(CookieJar, Json<serde_json::Value>)> {
    let token = jar
        .get("session")
        .map(|cookie| cookie.value().to_owned())
        .unwrap_or_default();
    let revoked = state.users.logout(&token).await?;
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

pub(crate) async fn list_users(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<bullastrator_storage::models::User>>> {
    Ok(Json(state.users.list_all_users().await?))
}

pub(crate) async fn list_roles(
    State(state): State<AppState>,
    query: axum::extract::Query<std::collections::HashMap<String, String>>,
) -> ApiResult<Json<Vec<bullastrator_storage::models::UserRole>>> {
    Ok(Json(state.roles.list(query.get("userId").map(String::as_str)).await?))
}

pub(crate) async fn create_role(
    State(state): State<AppState>,
    Json(request): Json<bullastrator_storage::models::CreateUserRole>,
) -> ApiResult<Json<bullastrator_storage::models::UserRole>> {
    Ok(Json(state.roles.create(request).await?))
}

pub(crate) async fn get_role(
    State(state): State<AppState>, Path(role_id): Path<String>,
) -> ApiResult<Json<bullastrator_storage::models::UserRole>> {
    Ok(Json(state.roles.get(&role_id).await?))
}

pub(crate) async fn update_role(
    State(state): State<AppState>, Path(role_id): Path<String>,
    Json(request): Json<bullastrator_storage::models::UpdateUserRole>,
) -> ApiResult<Json<bullastrator_storage::models::UserRole>> {
    Ok(Json(state.roles.update(&role_id, request).await?))
}

pub(crate) async fn delete_role(
    State(state): State<AppState>, Path(role_id): Path<String>,
) -> ApiResult<Json<u64>> {
    Ok(Json(state.roles.delete(&role_id).await?))
}

pub(crate) async fn role_options(
    State(state): State<AppState>,
    query: axum::extract::Query<std::collections::HashMap<String, String>>,
) -> ApiResult<Json<Vec<bullastrator_storage::models::RoleEntityOption>>> {
    let entity_type = query.get("entityType").map(String::as_str).unwrap_or("WORKSPACE");
    Ok(Json(state.roles.options(entity_type).await?))
}
