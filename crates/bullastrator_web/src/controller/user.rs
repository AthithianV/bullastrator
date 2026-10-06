use crate::controller::ApiResult;
use axum::{Json, extract::State};
use bullastrator_core::{
    models::user::{AuthResponse, LoginRequest, RegisterRequest, TokenRequest},
    state::AppState,
};

pub(crate) async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> ApiResult<Json<AuthResponse>> {
    Ok(Json(state.users.register(request).await?))
}

pub(crate) async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> ApiResult<Json<AuthResponse>> {
    Ok(Json(state.users.login(request).await?))
}

pub(crate) async fn logout(
    State(state): State<AppState>,
    Json(request): Json<TokenRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(
        serde_json::json!({"revoked": state.users.logout(&request.token).await?}),
    ))
}
