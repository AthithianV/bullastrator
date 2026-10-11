use bullastrator_core::state::AppState;
use bullastrator_storage::models::{CreateWorkspace, UpdateWorkspace, Workspace};
use tauri::{State, command};

async fn user_id(state: &AppState) -> Result<String, String> {
    state
        .users
        .ensure_default_user()
        .await
        .map(|user| user.id)
        .map_err(|error| error.to_string())
}

#[command]
pub async fn create_workspace(
    state: State<'_, AppState>,
    data: CreateWorkspace,
) -> Result<Workspace, String> {
    let user_id = user_id(&state).await?;
    state
        .workspaces
        .create(&user_id, data)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn get_all_workspaces(state: State<'_, AppState>) -> Result<Vec<Workspace>, String> {
    let user_id = user_id(&state).await?;
    state
        .workspaces
        .list_for_user(&user_id)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn get_workspace_by_id(
    state: State<'_, AppState>,
    id: String,
) -> Result<Option<Workspace>, String> {
    let user_id = user_id(&state).await?;
    state
        .workspaces
        .get(&id, &user_id)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn get_active_workspace(state: State<'_, AppState>) -> Result<Workspace, String> {
    let user_id = user_id(&state).await?;
    state
        .workspaces
        .active(&user_id)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn set_active_workspace(
    state: State<'_, AppState>,
    workspace_id: String,
) -> Result<Workspace, String> {
    let user_id = user_id(&state).await?;
    state
        .workspaces
        .select(&workspace_id, &user_id)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn update_workspace(
    state: State<'_, AppState>,
    id: String,
    data: UpdateWorkspace,
) -> Result<Workspace, String> {
    let user_id = user_id(&state).await?;
    state
        .workspaces
        .update(&id, &user_id, data)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn delete_workspace(state: State<'_, AppState>, id: String) -> Result<u64, String> {
    let user_id = user_id(&state).await?;
    state
        .workspaces
        .delete(&id, &user_id)
        .await
        .map_err(|error| error.to_string())
}
