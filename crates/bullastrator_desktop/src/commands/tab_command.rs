use bullastrator_core::state::AppState;
use bullastrator_storage::models::{CreateTab, Tab, UpdateTab};
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
pub async fn create_tab(state: State<'_, AppState>, data: CreateTab) -> Result<Tab, String> {
    let user_id = user_id(&state).await?;
    state
        .tabs
        .create(
            &user_id.clone(),
            CreateTab {
                id: data.id,
                connection_id: data.connection_id,
                user_id: Some(user_id),
                title: data.title,
                params: data.params,
            },
        )
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn update_tab(
    state: State<'_, AppState>,
    id: String,
    data: UpdateTab,
) -> Result<Tab, String> {
    let user_id = user_id(&state).await?;
    state
        .tabs
        .update(&id, &user_id, data)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn delete_tab(state: State<'_, AppState>, id: String) -> Result<String, String> {
    let user_id = user_id(&state).await?;
    state
        .tabs
        .delete(&id, &user_id)
        .await
        .map(|_| "Deleted successfully".to_owned())
        .map_err(|error| error.to_string())
}

#[command]
pub async fn get_tab(state: State<'_, AppState>, id: String) -> Result<Option<Tab>, String> {
    let user_id = user_id(&state).await?;
    state
        .tabs
        .get(&id, &user_id)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn get_all_tabs(state: State<'_, AppState>) -> Result<Vec<Tab>, String> {
    let user_id = user_id(&state).await?;
    state
        .tabs
        .list(&user_id)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn reorder_tabs(
    state: State<'_, AppState>,
    ordered_ids: Vec<String>,
) -> Result<(), String> {
    let user_id = user_id(&state).await?;
    state
        .tabs
        .reorder(&user_id, ordered_ids)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn set_active_tab(
    state: State<'_, AppState>,
    tab_id: Option<String>,
) -> Result<(), String> {
    let user_id = user_id(&state).await?;
    if let Some(tab_id) = tab_id {
        state
            .tabs
            .set_active(&tab_id, &user_id)
            .await
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[command]
pub async fn get_active_tab(state: State<'_, AppState>) -> Result<Option<String>, String> {
    let user_id = user_id(&state).await?;
    state
        .tabs
        .active(&user_id)
        .await
        .map_err(|error| error.to_string())
}
