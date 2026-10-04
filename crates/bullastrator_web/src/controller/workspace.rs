use crate::{controller::ApiResult, server::AppState};
use axum::{
    extract::{Path, State},
    Json,
};
use bullastrator_core::services::workspace::{CreateWorkspaceRequest, UpdateWorkspaceRequest};
use bullastrator_storage::models::{CreateWorkspaceMember, UpdateWorkspaceMember};
use serde::Deserialize;

pub(crate) async fn create(
    State(state): State<AppState>,
    Json(request): Json<CreateWorkspaceRequest>,
) -> ApiResult<Json<bullastrator_storage::models::Workspace>> {
    Ok(Json(state.workspaces.create(request).await?))
}

pub(crate) async fn get(
    State(state): State<AppState>,
    Path(workspace_id): Path<String>,
) -> ApiResult<Json<Option<bullastrator_storage::models::Workspace>>> {
    Ok(Json(state.workspaces.get(&workspace_id).await?))
}

pub(crate) async fn list_for_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> ApiResult<Json<Vec<bullastrator_storage::models::Workspace>>> {
    Ok(Json(state.workspaces.list_for_user(&user_id).await?))
}

pub(crate) async fn update(
    State(state): State<AppState>,
    Path(workspace_id): Path<String>,
    Json(request): Json<UpdateWorkspaceRequest>,
) -> ApiResult<Json<bullastrator_storage::models::Workspace>> {
    Ok(Json(state.workspaces.update(&workspace_id, request).await?))
}

pub(crate) async fn delete(
    State(state): State<AppState>,
    Path(workspace_id): Path<String>,
) -> ApiResult<Json<u64>> {
    Ok(Json(state.workspaces.delete(&workspace_id).await?))
}

pub(crate) async fn select(
    State(state): State<AppState>,
    Path(workspace_id): Path<String>,
) -> ApiResult<Json<bullastrator_storage::models::Workspace>> {
    Ok(Json(state.workspaces.select(&workspace_id).await?))
}

pub(crate) async fn active(
    State(state): State<AppState>,
) -> ApiResult<Json<bullastrator_storage::models::Workspace>> {
    Ok(Json(state.workspaces.active().await?))
}

pub(crate) async fn list_members(
    State(state): State<AppState>,
    Path(connection_id): Path<String>,
) -> ApiResult<Json<Vec<bullastrator_storage::models::WorkspaceMember>>> {
    Ok(Json(state.workspaces.list_members(&connection_id).await?))
}

#[derive(Debug, Deserialize)]
pub(crate) struct AddMemberRequest {
    user_id: String,
    role: String,
}

pub(crate) async fn add_member(
    State(state): State<AppState>,
    Path(connection_id): Path<String>,
    Json(request): Json<AddMemberRequest>,
) -> ApiResult<Json<bullastrator_storage::models::WorkspaceMember>> {
    Ok(Json(
        state
            .workspaces
            .add_member(CreateWorkspaceMember {
                user_id: request.user_id,
                connection_id,
                role: request.role,
            })
            .await?,
    ))
}

pub(crate) async fn update_member(
    State(state): State<AppState>,
    Path((connection_id, user_id)): Path<(String, String)>,
    Json(request): Json<UpdateWorkspaceMember>,
) -> ApiResult<Json<bullastrator_storage::models::WorkspaceMember>> {
    Ok(Json(
        state
            .workspaces
            .update_member(&user_id, &connection_id, request)
            .await?,
    ))
}

pub(crate) async fn remove_member(
    State(state): State<AppState>,
    Path((connection_id, user_id)): Path<(String, String)>,
) -> ApiResult<Json<u64>> {
    Ok(Json(
        state
            .workspaces
            .remove_member(&user_id, &connection_id)
            .await?,
    ))
}
