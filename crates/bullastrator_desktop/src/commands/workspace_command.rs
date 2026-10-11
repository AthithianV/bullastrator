use tauri::{State, command};
use tracing::error;
use uuid::Uuid;

use crate::model::workspace_model::{
    CreateWorkspaceModel, ReadWorkspaceModel, UpdateWorkspaceModel,
};
use crate::repository::workspace_repository::WorkspaceRepository;
use crate::utilities::app_state::AppState;

#[command]
pub async fn create_workspace(
    app_state: State<'_, AppState>,
    data: CreateWorkspaceModel,
) -> Result<ReadWorkspaceModel, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let workspace_repo = WorkspaceRepository::new(&db_conn);

    workspace_repo.create(data).await.map_err(|e| {
        error!("Failed to create workspace: {:?}", e);
        e.to_string()
    })
}

#[command]
pub async fn get_all_workspaces(
    app_state: State<'_, AppState>,
) -> Result<Vec<ReadWorkspaceModel>, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let workspace_repo = WorkspaceRepository::new(&db_conn);

    workspace_repo.get_all().await.map_err(|e| {
        error!("Failed to fetch all workspaces: {:?}", e);
        e.to_string()
    })
}

#[command]
pub async fn get_workspace_by_id(
    app_state: State<'_, AppState>,
    id: Uuid,
) -> Result<ReadWorkspaceModel, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let workspace_repo = WorkspaceRepository::new(&db_conn);

    workspace_repo.get_by_id(id).await.map_err(|e| {
        error!("Failed to fetch workspace {}: {:?}", id, e);
        e.to_string()
    })
}

#[command]
pub async fn get_active_workspace(
    app_state: State<'_, AppState>,
) -> Result<ReadWorkspaceModel, String> {
    let active_workspace = app_state.get_active_workspace().await?;

    Ok(active_workspace)
}

#[command]
pub async fn set_active_workspace(
    app_state: State<'_, AppState>,
    workspace_id: Uuid,
) -> Result<(), String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let workspace_repo = WorkspaceRepository::new(&db_conn);

    let workspace = workspace_repo.get_by_id(workspace_id).await.map_err(|e| {
        error!("Failed to fetch workspace {}: {:?}", workspace_id, e);
        e.to_string()
    })?;

    workspace_repo
        .set_active_workspace(workspace_id)
        .await
        .map_err(|e| {
            error!("Failed to set active workspace {}: {:?}", workspace_id, e);
            e.to_string()
        })?;

    app_state.set_active_workspace(workspace).await;

    Ok(())
}

#[command]
pub async fn update_workspace(
    app_state: State<'_, AppState>,
    id: Uuid,
    data: UpdateWorkspaceModel,
) -> Result<ReadWorkspaceModel, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let workspace_repo = WorkspaceRepository::new(&db_conn);

    workspace_repo.update(id, data).await.map_err(|e| {
        error!("Failed to update workspace {}: {:?}", id, e);
        e.to_string()
    })
}

#[command]
pub async fn delete_workspace(app_state: State<'_, AppState>, id: Uuid) -> Result<String, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let workspace_repo = WorkspaceRepository::new(&db_conn);

    workspace_repo
        .delete(id)
        .await
        .map(|_| "Workspace deleted successfully".to_string())
        .map_err(|e| {
            error!("Failed to delete workspace {}: {:?}", id, e);
            e.to_string()
        })
}
