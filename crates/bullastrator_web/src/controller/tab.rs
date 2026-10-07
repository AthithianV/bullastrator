use crate::{controller::ApiResult, models::user::AuthenticatedUser};
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use bullastrator_core::state::AppState;
use bullastrator_storage::models::{CreateTab, Tab, UpdateTab};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ReorderTabsRequest {
    pub ordered_ids: Vec<String>,
}

pub(crate) async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<CreateTab>,
) -> ApiResult<Json<Tab>> {
    Ok(Json(state.tabs.create(&user.id, request).await?))
}

pub(crate) async fn list(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> ApiResult<Json<Vec<Tab>>> {
    Ok(Json(state.tabs.list(&user.id).await?))
}

pub(crate) async fn get(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(tab_id): Path<String>,
) -> ApiResult<Json<Option<Tab>>> {
    Ok(Json(state.tabs.get(&tab_id, &user.id).await?))
}

pub(crate) async fn update(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(tab_id): Path<String>,
    Json(request): Json<UpdateTab>,
) -> ApiResult<Json<Tab>> {
    Ok(Json(state.tabs.update(&tab_id, &user.id, request).await?))
}

pub(crate) async fn delete(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(tab_id): Path<String>,
) -> ApiResult<Json<u64>> {
    Ok(Json(state.tabs.delete(&tab_id, &user.id).await?))
}

pub(crate) async fn set_active(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(tab_id): Path<String>,
) -> ApiResult<Json<()>> {
    state.tabs.set_active(&tab_id, &user.id).await?;
    Ok(Json(()))
}

pub(crate) async fn active(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> ApiResult<Json<Option<String>>> {
    Ok(Json(state.tabs.active(&user.id).await?))
}

pub(crate) async fn reorder(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<ReorderTabsRequest>,
) -> ApiResult<Json<()>> {
    state.tabs.reorder(&user.id, request.ordered_ids).await?;
    Ok(Json(()))
}
