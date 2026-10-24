use bullastrator_core::{
    models::folder::ReadQueueWithCounts,
    state::AppState,
};
use bullastrator_storage::models::{CreateFolder, Folder, FolderWithQueues, UpdateFolder};
use tauri::{command, State};

async fn user_id(state: &AppState) -> Result<String, String> {
    state
        .users
        .ensure_default_user()
        .await
        .map(|user| user.id)
        .map_err(|error| error.to_string())
}

#[command]
pub async fn create_folder(state: State<'_, AppState>, data: CreateFolder) -> Result<Folder, String> {
    let user_id = user_id(&state).await?;
    state.folders.create(&user_id, data).await.map_err(|error| error.to_string())
}

#[command]
pub async fn get_all_folders(state: State<'_, AppState>, connection_id: String) -> Result<Vec<FolderWithQueues>, String> {
    let user_id = user_id(&state).await?;
    state.folders.list(&connection_id, &user_id).await.map_err(|error| error.to_string())
}

#[command]
pub async fn get_folder_by_id(state: State<'_, AppState>, id: String) -> Result<Option<Folder>, String> {
    let user_id = user_id(&state).await?;
    state.folders.get(&id, &user_id).await.map_err(|error| error.to_string())
}

#[command]
pub async fn update_folder(state: State<'_, AppState>, id: String, title: String) -> Result<Folder, String> {
    let user_id = user_id(&state).await?;
    state
        .folders
        .update(&id, &user_id, UpdateFolder { title })
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn delete_folder(state: State<'_, AppState>, id: String) -> Result<String, String> {
    let user_id = user_id(&state).await?;
    state
        .folders
        .delete(&id, &user_id)
        .await
        .map(|_| "Folder deleted successfully".to_owned())
        .map_err(|error| error.to_string())
}

#[command]
pub async fn toggle_queue_in_folder(
    state: State<'_, AppState>,
    folder_id: String,
    queue_id: String,
) -> Result<bool, String> {
    let user_id = user_id(&state).await?;
    state
        .folders
        .toggle_queue(&folder_id, &queue_id, &user_id)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn reorder_folder_queues(
    state: State<'_, AppState>,
    folder_id: String,
    ordered_queue_ids: Vec<String>,
) -> Result<(), String> {
    let user_id = user_id(&state).await?;
    state
        .folders
        .reorder(&folder_id, ordered_queue_ids, &user_id)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn reorder_folders(
    state: State<'_, AppState>,
    connection_id: String,
    ordered_folder_ids: Vec<String>,
) -> Result<(), String> {
    let user_id = user_id(&state).await?;
    state
        .folders
        .reorder_folders(&connection_id, ordered_folder_ids, &user_id)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn get_queues_for_folder(
    state: State<'_, AppState>,
    folder_id: String,
) -> Result<Vec<ReadQueueWithCounts>, String> {
    let user_id = user_id(&state).await?;
    let folder = state
        .folders
        .get(&folder_id, &user_id)
        .await
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "Folder not found".to_owned())?;
    let redis = state
        .get_redis_connection(&folder.connection_id)
        .await
        .map_err(|error| error.to_string())?;
    state
        .folders
        .queues(&folder_id, &user_id, &redis)
        .await
        .map_err(|error| error.to_string())
}
