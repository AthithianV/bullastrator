use anyhow::Result;
use tauri::{State, command};
use tracing::error;
use uuid::Uuid;

use crate::model::connection_model::{
    ConnectionHealth, CreateConnectionModel, ReadConnectionModel, UpdateConnectionModel,
};
use crate::repository::connection_repository::ConnectionRepository;
use crate::service::connection_service::{
    check_redis_version, start_health_check_service, test_redis_connection_service,
};
use crate::utilities::app_state::AppState;
use crate::utilities::redis_utils::{create_redis_url, get_or_create_redis_connection};

#[command]
pub async fn create_connection(
    app_state: State<'_, AppState>,
    data: CreateConnectionModel,
) -> Result<ReadConnectionModel, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let workspace = app_state.get_active_workspace().await?;

    let connection_repo = ConnectionRepository::new(&db_conn);

    let _ = check_redis_version(&data).await.map_err(|e| {
        error!("{:?}", e);
        e.to_string()
    });

    let new_connection = connection_repo
        .create(data.clone(), workspace.id)
        .await
        .map_err(|e| {
            error!("{:?}", e);
            e.to_string()
        });

    new_connection
}

#[command]
pub async fn update_connection(
    app_state: State<'_, AppState>,
    id: Uuid,
    data: UpdateConnectionModel,
) -> Result<ReadConnectionModel, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let connection_repo = ConnectionRepository::new(&db_conn);

    let existing = connection_repo
        .get_by_id_with_password(id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?
        .ok_or_else(|| "Connection not found".to_string())?;

    let check_data = CreateConnectionModel {
        host: data.host.clone().unwrap_or(existing.host),
        port: data.port.unwrap_or(existing.port),
        username: data.username.clone().or(existing.username),
        password: data.password.clone().or(existing.password),
        db: Some(data.db.unwrap_or(existing.db)),
        is_tls_enabled: data.is_tls_enabled.unwrap_or(existing.is_tls_enabled),
        bullmq_prefix: Some(existing.bullmq_prefix),
        color: data.color.clone().or(existing.color),
        label: data.label.clone().or(existing.label),
        name: existing.name,
    };

    // 3. Perform the Version Check (with the timeout we discussed)
    check_redis_version(&check_data).await.map_err(|e| {
        error!("Redis Version Check Failed: {:?}", e);
        format!("Compatibility Error: {}", e)
    })?;

    let result = connection_repo.update(id, data).await.map_err(|e| {
        error!("An error occurred: {:?}", e);
        e.to_string()
    });

    app_state.remove_redis_connection(&id).await;

    result
}

#[command]
pub async fn delete_connection(app_state: State<'_, AppState>, id: Uuid) -> Result<String, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let connection_repo = ConnectionRepository::new(&db_conn);

    let result = connection_repo
        .delete(id)
        .await
        .map(|_| "Deleted successfully".to_string())
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        });

    app_state.remove_redis_connection(&id).await;

    result
}

#[command]
pub async fn get_connection(
    app_state: State<'_, AppState>,
    id: Uuid,
) -> Result<Option<ReadConnectionModel>, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let connection_repo = ConnectionRepository::new(&db_conn);

    connection_repo.get_by_id(id).await.map_err(|e| {
        error!("An error occurred: {:?}", e);
        e.to_string()
    })
}

#[command]
pub async fn get_all_connections(
    app_state: State<'_, AppState>,
) -> Result<Vec<ReadConnectionModel>, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let active_workspace = app_state.get_active_workspace().await?;

    let connection_repo = ConnectionRepository::new(&db_conn);

    connection_repo
        .get_all(active_workspace.id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })
}

#[command]
pub async fn start_health_check(
    app_state: State<'_, AppState>,
    connection_id: Uuid,
) -> Result<bool, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    start_health_check_service(&connection_pool)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })
}

#[command]
pub async fn check_health_for_all_connections(
    app_state: State<'_, AppState>,
) -> Result<Vec<ConnectionHealth>, String> {
    let db_conn = app_state.get_app_db_connection().await?;

    let active_workspace = app_state.get_active_workspace().await?;

    let connection_repo = ConnectionRepository::new(&db_conn);

    let connections = connection_repo
        .get_all(active_workspace.id)
        .await
        .map_err(|e| {
            error!("An error occurred getting all connects: {:?}", e);
            e.to_string()
        })?;

    let mut connections_status = Vec::new();
    for connection in &connections {
        let connection_pool = get_or_create_redis_connection(&app_state, connection.id)
            .await
            .map_err(|e| {
                error!(
                    "An error occurred while connecting {}: {:?}",
                    connection.name, e
                );
                e.to_string()
            })?;

        let connection_status = start_health_check_service(&connection_pool).await;

        connections_status.push(ConnectionHealth {
            id: connection.id,
            is_active: connection_status.unwrap_or_default(),
        });
    }

    Ok(connections_status)
}

#[command]
pub async fn test_redis_connection(
    host: String,
    port: i32,
    username: Option<String>,
    password: Option<String>,
    db: i32,
    is_tls_enabled: bool,
) -> Result<bool, String> {
    let redis_url = create_redis_url(host, port, username, password, db, is_tls_enabled)
        .map_err(|e| e.to_string())?;

    test_redis_connection_service(redis_url).await
}
