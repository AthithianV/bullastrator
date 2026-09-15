use crate::model::folder_model::{CreateFolderModel, ReadFolderModel, ReadFolderWithQueuesModel};
use crate::model::queue_model::{QueueJobCounts, ReadQueueWithCounts};
use crate::repository::folder_repository::FolderRepository;
use crate::service::job_service::JobService;
use crate::utilities::app_state::AppState;
use crate::utilities::redis_utils::get_or_create_redis_connection;
use tauri::{State, command};
use tracing::error;
use uuid::Uuid;

#[command]
pub async fn create_folder(
    app_state: State<'_, AppState>,
    data: CreateFolderModel,
) -> Result<ReadFolderModel, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let folder_repository = FolderRepository::new(&db_conn);

    let new_connection = folder_repository.create(&data).await.map_err(|e| {
        error!("{:?}", e);
        e.to_string()
    });

    new_connection
}

#[command]
pub async fn get_all_folders(
    app_state: State<'_, AppState>,
    connection_id: Uuid,
) -> Result<Vec<ReadFolderWithQueuesModel>, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let folder_repo = FolderRepository::new(&db_conn);

    folder_repo.get_all(connection_id).await.map_err(|e| {
        error!("Failed to get folders: {:?}", e);
        e.to_string()
    })
}

#[command]
pub async fn get_folder_by_id(
    app_state: State<'_, AppState>,
    id: Uuid,
) -> Result<Option<ReadFolderModel>, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let folder_repo = FolderRepository::new(&db_conn);

    folder_repo.get_by_id(id).await.map_err(|e| {
        error!("Failed to get folder {}: {:?}", id, e);
        e.to_string()
    })
}

#[command]
pub async fn update_folder(
    app_state: State<'_, AppState>,
    id: Uuid,
    title: String,
) -> Result<ReadFolderModel, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let folder_repo = FolderRepository::new(&db_conn);

    folder_repo.update(id, title).await.map_err(|e| {
        error!("Failed to update folder {}: {:?}", id, e);
        e.to_string()
    })
}

#[command]
pub async fn delete_folder(app_state: State<'_, AppState>, id: Uuid) -> Result<String, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let folder_repo = FolderRepository::new(&db_conn);

    folder_repo
        .delete(id)
        .await
        .map(|_| "Folder deleted successfully".to_string())
        .map_err(|e| {
            error!("Failed to delete folder {}: {:?}", id, e);
            e.to_string()
        })
}

// --- Folder Queue Management ---

#[command]
pub async fn toggle_queue_in_folder(
    app_state: State<'_, AppState>,
    folder_id: Uuid,
    queue_id: Uuid,
) -> Result<bool, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let folder_repo = FolderRepository::new(&db_conn);

    folder_repo
        .toggle_queue_in_folder(folder_id, queue_id)
        .await
        .map_err(|e| {
            error!(
                "Failed to toggle queue {} in folder {}: {:?}",
                queue_id, folder_id, e
            );
            e.to_string()
        })
}

#[command]
pub async fn reorder_folder_queues(
    app_state: State<'_, AppState>,
    folder_id: Uuid,
    ordered_queue_ids: Vec<Uuid>,
) -> Result<(), String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let folder_repo = FolderRepository::new(&db_conn);

    folder_repo
        .reorder_queues(folder_id, ordered_queue_ids)
        .await
        .map_err(|e| {
            error!("Failed to reorder queues for folder {}: {:?}", folder_id, e);
            e.to_string()
        })
}

#[command]
pub async fn get_queues_for_folder(
    app_state: State<'_, AppState>,
    folder_id: Uuid,
) -> Result<Vec<ReadQueueWithCounts>, String> {
    let db_conn = app_state.get_app_db_connection().await?;
    let folder_repo = FolderRepository::new(&db_conn);

    let queues_of_folders = folder_repo
        .get_queues_for_folder(folder_id)
        .await
        .map_err(|e| {
            error!("Failed to get queues for folder {}: {:?}", folder_id, e);
            e.to_string()
        })?;

    if queues_of_folders.is_empty() {
        return Ok(vec![]);
    }

    let queue_names: Vec<&str> = queues_of_folders
        .iter()
        .map(|q| q.queue_name.as_str())
        .collect();

    let connection_pool =
        get_or_create_redis_connection(&app_state, queues_of_folders[0].connection_id)
            .await
            .map_err(|e| {
                error!("An error occurred: {:?}", e);
                e.to_string()
            })?;

    let job_service = JobService::new(&connection_pool);
    let job_counts = job_service
        .get_job_count_in_queue_service(queue_names)
        .await
        .map_err(|e| {
            error!("An error occurred while getting count in queue: {:?}", e);
            e.to_string()
        })?;

    let counts_map: std::collections::HashMap<String, QueueJobCounts> = job_counts
        .into_iter()
        .map(|c| (c.queue_name.clone(), c))
        .collect();

    // 4. Merge them into the combined struct
    let result: Vec<ReadQueueWithCounts> = queues_of_folders
        .into_iter()
        .map(|q| {
            let name = q.queue_name.clone();
            ReadQueueWithCounts {
                counts: counts_map.get(&name).cloned().unwrap_or(QueueJobCounts {
                    queue_name: name,
                    ..Default::default()
                }),
                queue: q,
            }
        })
        .collect();

    Ok(result)
}
