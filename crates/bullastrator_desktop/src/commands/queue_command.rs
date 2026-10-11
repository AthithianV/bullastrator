use bullastrator_core::state::AppState;
use bullastrator_storage::models::{ConnectionWithQueues, Queue};
use bullastrator_core::services::queue::QueueDetails;
use tauri::{command, State};

async fn user_id(state: &AppState) -> Result<String, String> {
    state.users.ensure_default_user().await.map(|user| user.id).map_err(|error| error.to_string())
}

#[command]
pub async fn sync_all_queue_names(state: State<'_, AppState>, connection_id: String) -> Result<(), String> {
    let user_id = user_id(&state).await?;
    let service = state.get_queue_service(&connection_id).await.map_err(|error| error.to_string())?;
    service.sync_queues(&user_id).await.map(|_| ()).map_err(|error| error.to_string())
}

#[command]
pub async fn get_all_queues_by_connection(state: State<'_, AppState>, connection_id: String) -> Result<Vec<Queue>, String> {
    let user_id = user_id(&state).await?;
    let service = state.get_queue_service(&connection_id).await.map_err(|error| error.to_string())?;
    service.list_queues(&user_id).await.map_err(|error| error.to_string())
}

#[command]
pub async fn get_all_queues_by_workspace(state: State<'_, AppState>) -> Result<Vec<ConnectionWithQueues>, String> {
    let user_id = user_id(&state).await?;
    state.connections.list_queues(&user_id).await.map_err(|error| error.to_string())
}

#[command]
pub async fn pause_queue(state: State<'_, AppState>, connection_id: String, queue_name: String, should_pause: bool) -> Result<String, String> {
    let user_id = user_id(&state).await?;
    let service = state.get_queue_service(&connection_id).await.map_err(|error| error.to_string())?;
    service.pause_queue_service(&queue_name, should_pause, &user_id).await.map_err(|error| error.to_string())
}

#[command]
pub async fn get_queue_details(state: State<'_, AppState>, connection_id: String, queue_name: String) -> Result<QueueDetails, String> {
    let user_id = user_id(&state).await?;
    let service = state.get_queue_service(&connection_id).await.map_err(|error| error.to_string())?;
    service.get_queue_details_service(&queue_name, &user_id).await.map_err(|error| error.to_string())
}
