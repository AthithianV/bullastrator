use crate::model::queue_model::{ConnectionWithQueues, QueueDetails, ReadQueueModel};
use crate::repository::queue_repository::QueueRepository;
use crate::service::queue_service::QueueService;
use crate::utilities::app_state::AppState;
use crate::utilities::redis_utils::get_or_create_redis_connection;
use tauri::{State, command};
use tracing::error;
use uuid::Uuid;

#[command]
pub async fn sync_all_queue_names(
    app_state: State<'_, AppState>,
    connection_id: Uuid,
) -> Result<(), String> {
    #[cfg(debug_assertions)]
    {
        use std::{thread, time::Duration};

        println!("Dev mode: Sleeping for 5 second...");
        thread::sleep(Duration::from_secs(5));
    }

    let db_conn = app_state.get_app_db_connection().await?;

    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })
        .unwrap();

    let queue_service = QueueService::new(&connection_pool);
    // Step 2: Pass redis connection to fetch_all_queues function to get vector of all queue name.
    queue_service
        .get_all_bullmq_queues(db_conn)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    Ok(())
}

#[command]
pub async fn get_all_queues_by_connection(
    app_state: State<'_, AppState>,
    connection_id: Uuid,
) -> Result<Vec<ReadQueueModel>, String> {
    let db_conn = app_state.get_app_db_connection().await?;

    // Step 1: Get the connection from connection table.
    let queue_repository = QueueRepository::new(&db_conn);
    let queues = queue_repository
        .get_all_by_connection(connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })
        .unwrap();

    Ok(queues)
}

#[command]
pub async fn get_all_queues_by_workspace(
    app_state: State<'_, AppState>,
) -> Result<Vec<ConnectionWithQueues>, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let workspace = app_state.get_active_workspace().await?;

    // Step 1: Get the connection from connection table.
    let queue_repository = QueueRepository::new(&db_conn);
    let queues = queue_repository
        .get_all_by_workspace(workspace.id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })?;

    Ok(queues)
}

#[command]
pub async fn pause_queue(
    app_state: State<'_, AppState>,
    connection_id: Uuid,
    queue_name: String,
    should_pause: bool,
) -> Result<String, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })
        .unwrap();

    let queue_service = QueueService::new(&connection_pool);

    queue_service
        .pause_queue_service(queue_name, should_pause)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })
}

#[command]
pub async fn get_queue_details(
    app_state: State<'_, AppState>,
    connection_id: Uuid,
    queue_name: String,
) -> Result<QueueDetails, String> {
    let connection_pool = get_or_create_redis_connection(&app_state, connection_id)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })
        .unwrap();

    let queue_service = QueueService::new(&connection_pool);

    queue_service
        .get_queue_details_service(queue_name)
        .await
        .map_err(|e| {
            error!("An error occurred: {:?}", e);
            e.to_string()
        })
}
