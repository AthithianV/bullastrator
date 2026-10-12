use bullastrator_core::{services::connection, state::AppState};
use bullastrator_storage::models::{
    Connection, ConnectionDetails, CreateConnection, UpdateConnection,
};
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
pub async fn create_connection(
    state: State<'_, AppState>,
    data: CreateConnection,
) -> Result<Connection, String> {
    let user_id = user_id(&state).await?;
    connection::check_redis_version(&data)
        .await
        .map_err(|error| error.to_string())?;
    state
        .connections
        .create(&user_id, data)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn update_connection(
    state: State<'_, AppState>,
    id: String,
    data: UpdateConnection,
) -> Result<Connection, String> {
    let user_id = user_id(&state).await?;
    state
        .connections
        .update(&id, &user_id, data)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn delete_connection(state: State<'_, AppState>, id: String) -> Result<String, String> {
    let user_id = user_id(&state).await?;
    state
        .connections
        .delete(&id, &user_id)
        .await
        .map(|_| "Deleted successfully".to_owned())
        .map_err(|error| error.to_string())
}

#[command]
pub async fn get_connection(
    state: State<'_, AppState>,
    id: String,
) -> Result<Option<ConnectionDetails>, String> {
    let user_id = user_id(&state).await?;
    state
        .connections
        .get(&id, &user_id)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn get_all_connections(state: State<'_, AppState>) -> Result<Vec<Connection>, String> {
    let user_id = user_id(&state).await?;
    state
        .connections
        .list(&user_id)
        .await
        .map_err(|error| error.to_string())
}

#[command]
pub async fn start_health_check(
    state: State<'_, AppState>,
    connection_id: String,
) -> Result<bool, String> {
    let user_id = user_id(&state).await?;
    state
        .connections
        .authorize(
            &connection_id,
            &user_id,
            bullastrator_storage::models::WorkspaceRole::VIEWER,
        )
        .await
        .map_err(|error| error.to_string())?;
    let redis = state
        .get_redis_connection(&connection_id)
        .await
        .map_err(|error| error.to_string())?;
    connection::start_health_check_service(&redis.pool)
        .await
        .map_err(|error| error.to_string())
}

// #[command]
// pub async fn check_health_for_all_connections(
//     state: State<'_, AppState>,
// ) -> Result<Vec<ConnectionHealth>, String> {
//     let user_id = user_id(&state).await?;
//     let connections = state
//         .connections
//         .list(&user_id)
//         .await
//         .map_err(|error| error.to_string())?;
//     let mut result = Vec::with_capacity(connections.len());
//     for item in connections {
//         let active = match state.get_redis_connection(&item.id).await {
//             Ok(redis) => connection::start_health_check_service(&redis.pool)
//                 .await
//                 .unwrap_or(false),
//             Err(_) => false,
//         };
//         result.push(ConnectionHealth {
//             id: item.id,
//             is_active: active,
//         });
//     }
//     Ok(result)
// }

#[command]
pub async fn test_redis_connection(
    host: String,
    port: i32,
    username: Option<String>,
    password: Option<String>,
    db: i32,
    is_tls_enabled: bool,
) -> Result<bool, String> {
    connection::test_redis_connection_service(&CreateConnection {
        name: String::new(),
        host,
        port,
        username,
        password,
        db: Some(db),
        bullmq_prefix: None,
        color: None,
        label: None,
        is_tls_enabled,
    })
    .await
    .map_err(|error| error.to_string())
}
