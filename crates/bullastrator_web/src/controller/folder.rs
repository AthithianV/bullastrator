use crate::{controller::ApiResult, models::user::AuthenticatedUser};
use anyhow::Context;
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use bullastrator_core::state::AppState;
use bullastrator_storage::models::{CreateFolder, Folder, FolderWithQueues, UpdateFolder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReorderFolderQueuesRequest {
    pub ordered_queue_ids: Vec<String>,
}

pub(crate) async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<CreateFolder>,
) -> ApiResult<Json<Folder>> {
    Ok(Json(state.folders.create(&user.id, request).await?))
}

pub(crate) async fn list(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(connection_id): Path<String>,
) -> ApiResult<Json<Vec<FolderWithQueues>>> {
    Ok(Json(state.folders.list(&connection_id, &user.id).await?))
}

pub(crate) async fn get(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(folder_id): Path<String>,
) -> ApiResult<Json<Option<Folder>>> {
    Ok(Json(state.folders.get(&folder_id, &user.id).await?))
}

pub(crate) async fn update(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(folder_id): Path<String>,
    Json(request): Json<UpdateFolder>,
) -> ApiResult<Json<Folder>> {
    Ok(Json(
        state.folders.update(&folder_id, &user.id, request).await?,
    ))
}

pub(crate) async fn delete(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(folder_id): Path<String>,
) -> ApiResult<Json<u64>> {
    Ok(Json(state.folders.delete(&folder_id, &user.id).await?))
}

pub(crate) async fn queues(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(folder_id): Path<String>,
) -> ApiResult<Json<Vec<bullastrator_core::models::folder::ReadQueueWithCounts>>> {
    let folder = state
        .folders
        .get(&folder_id, &user.id)
        .await?
        .context("Folder not found")?;
    let redis = state.get_redis_connection(&folder.connection_id).await?;
    Ok(Json(
        state.folders.queues(&folder_id, &user.id, &redis).await?,
    ))
}

pub(crate) async fn toggle_queue(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((folder_id, queue_id)): Path<(String, String)>,
) -> ApiResult<Json<bool>> {
    Ok(Json(
        state
            .folders
            .toggle_queue(&folder_id, &queue_id, &user.id)
            .await?,
    ))
}

pub(crate) async fn reorder(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(folder_id): Path<String>,
    Json(request): Json<ReorderFolderQueuesRequest>,
) -> ApiResult<Json<()>> {
    state
        .folders
        .reorder(&folder_id, request.ordered_queue_ids, &user.id)
        .await?;
    Ok(Json(()))
}
