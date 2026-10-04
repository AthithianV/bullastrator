use crate::{
    controller::ApiResult,
    models::{user::AuthenticatedUser, workspace::AddMemberRequest},
};
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use bullastrator_core::state::AppState;
use bullastrator_storage::models::{
    CreateWorkspace, CreateWorkspaceMember, UpdateWorkspace, UpdateWorkspaceMember, Workspace,
    WorkspaceMember,
};

pub(crate) async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<CreateWorkspace>,
) -> ApiResult<Json<Workspace>> {
    Ok(Json(state.workspaces.create(&user.id, request).await?))
}

pub(crate) async fn get(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(workspace_id): Path<String>,
) -> ApiResult<Json<Option<Workspace>>> {
    Ok(Json(state.workspaces.get(&workspace_id, &user.id).await?))
}

pub(crate) async fn list_for_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> ApiResult<Json<Vec<Workspace>>> {
    Ok(Json(state.workspaces.list_for_user(&user_id).await?))
}

pub(crate) async fn update(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(workspace_id): Path<String>,
    Json(request): Json<UpdateWorkspace>,
) -> ApiResult<Json<Workspace>> {
    Ok(Json(
        state
            .workspaces
            .update(&workspace_id, &user.id, request)
            .await?,
    ))
}

pub(crate) async fn delete(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(workspace_id): Path<String>,
) -> ApiResult<Json<u64>> {
    Ok(Json(
        state.workspaces.delete(&workspace_id, &user.id).await?,
    ))
}

pub(crate) async fn select(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(workspace_id): Path<String>,
) -> ApiResult<Json<Workspace>> {
    Ok(Json(
        state.workspaces.select(&workspace_id, &user.id).await?,
    ))
}

pub(crate) async fn active(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> ApiResult<Json<Workspace>> {
    Ok(Json(state.workspaces.active(&user.id).await?))
}

pub(crate) async fn list_members(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(workspace_id): Path<String>,
) -> ApiResult<Json<Vec<WorkspaceMember>>> {
    Ok(Json(
        state
            .workspaces
            .list_members(&workspace_id, &user.id)
            .await?,
    ))
}

pub(crate) async fn add_member(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(workspace_id): Path<String>,
    Json(request): Json<AddMemberRequest>,
) -> ApiResult<Json<WorkspaceMember>> {
    Ok(Json(
        state
            .workspaces
            .add_member(
                &workspace_id.clone(),
                &user.id,
                CreateWorkspaceMember {
                    user_id: request.user_id,
                    workspace_id: workspace_id,
                    role: request.role,
                },
            )
            .await?,
    ))
}

pub(crate) async fn update_member(
    State(state): State<AppState>,
    Path((workspace_id, user_id)): Path<(String, String)>,
    Json(request): Json<UpdateWorkspaceMember>,
) -> ApiResult<Json<WorkspaceMember>> {
    Ok(Json(
        state
            .workspaces
            .update_member(&user_id, &workspace_id, request)
            .await?,
    ))
}

pub(crate) async fn remove_member(
    State(state): State<AppState>,
    Path((workspace_id, user_id)): Path<(String, String)>,
) -> ApiResult<Json<u64>> {
    Ok(Json(
        state
            .workspaces
            .remove_member(&user_id, &workspace_id)
            .await?,
    ))
}
