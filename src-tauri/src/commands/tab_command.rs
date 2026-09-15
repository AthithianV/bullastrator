use tauri::{State, command};
use tracing::error;

use crate::model::tab_model::{CreateTabModel, ReadTabModel, UpdateTabModel};
use crate::repository::tab_repository::TabRepository;
use crate::utilities::app_state::AppState;

#[command]
pub async fn create_tab(
    app_state: State<'_, AppState>,
    id: String,
    data: CreateTabModel,
) -> Result<ReadTabModel, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let workspace = app_state.get_active_workspace().await?;

    let tab_repo = TabRepository::new(&db_conn);

    tab_repo.create(id, workspace.id, data).await.map_err(|e| {
        error!("Failed to create tab: {:?}", e);
        e.to_string()
    })
}

#[command]
pub async fn update_tab(
    app_state: State<'_, AppState>,
    id: String,
    data: UpdateTabModel,
) -> Result<ReadTabModel, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let tab_repo = TabRepository::new(&db_conn);

    tab_repo.update(id, data).await.map_err(|e| {
        error!("Failed to update tab: {:?}", e);
        e.to_string()
    })
}

#[command]
pub async fn delete_tab(app_state: State<'_, AppState>, id: String) -> Result<String, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let tab_repo = TabRepository::new(&db_conn);

    tab_repo
        .delete(id)
        .await
        .map(|_| "Deleted successfully".to_string())
        .map_err(|e| {
            error!("Failed to delete tab: {:?}", e);
            e.to_string()
        })
}

#[command]
pub async fn get_tab(
    app_state: State<'_, AppState>,
    id: String,
) -> Result<Option<ReadTabModel>, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let tab_repo = TabRepository::new(&db_conn);

    tab_repo.get_by_id(id).await.map_err(|e| {
        error!("Failed to get tab: {:?}", e);
        e.to_string()
    })
}

#[command]
pub async fn get_all_tabs(app_state: State<'_, AppState>) -> Result<Vec<ReadTabModel>, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let active_workspace = app_state.get_active_workspace().await?;

    let tab_repo = TabRepository::new(&db_conn);

    // Pass the active workspace ID to filter tabs
    tab_repo.get_all(active_workspace.id).await.map_err(|e| {
        error!("Failed to fetch all tabs: {:?}", e);
        e.to_string()
    })
}

// Optional: If you implemented the reorder functionality discussed previously
#[command]
pub async fn reorder_tabs(
    app_state: State<'_, AppState>,
    ordered_ids: Vec<String>,
) -> Result<(), String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let workspace = app_state.get_active_workspace().await?;
    let tab_repo = TabRepository::new(&db_conn);

    tab_repo
        .update_order(workspace.id, ordered_ids)
        .await
        .map_err(|e| {
            error!("Failed to reorder tabs: {:?}", e);
            e.to_string()
        })
}

#[command]
pub async fn set_active_tab(
    app_state: State<'_, AppState>,
    tab_id: Option<String>,
) -> Result<(), String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let workspace = app_state.get_active_workspace().await?;
    let tab_repo = TabRepository::new(&db_conn);

    tab_repo
        .set_active_tab(workspace.id, tab_id)
        .await
        .map_err(|e| {
            error!("Failed to set active tab: {:?}", e);
            e.to_string()
        })
}

#[command]
pub async fn get_active_tab(app_state: State<'_, AppState>) -> Result<Option<String>, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let workspace = app_state.get_active_workspace().await?;
    let tab_repo = TabRepository::new(&db_conn);

    tab_repo.get_active_tab(workspace.id).await.map_err(|e| {
        error!("Failed to set active tab: {:?}", e);
        e.to_string()
    })
}
