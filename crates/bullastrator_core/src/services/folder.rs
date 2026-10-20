use anyhow::{Context, Result, bail};
use bullastrator_storage::{
    models::{
        CreateFolder, CreateTab, Folder, FolderWithQueues, UpdateFolder, UpdateTab,
        WorkspaceRole,
    },
    repositories::FolderRepository,
};

use crate::{
    models::{folder::ReadQueueWithCounts, job::QueueJobCounts},
    services::{RedisConnection, job::JobService},
};

use super::{connection::ConnectionService, tab::TabService};

#[derive(Clone)]
pub struct FolderService {
    repository: FolderRepository,
    connections: ConnectionService,
    tabs: TabService,
}

impl FolderService {
    pub fn new(
        repository: FolderRepository,
        connections: ConnectionService,
        tabs: TabService,
    ) -> Self {
        Self {
            repository,
            connections,
            tabs,
        }
    }

    async fn folder_for_user(&self, folder_id: &str, user_id: &str) -> Result<Folder> {
        let folder = self
            .repository
            .get_by_id(folder_id)
            .await?
            .context("Folder not found")?;
        self.connections
            .authorize(&folder.connection_id, user_id, WorkspaceRole::VIEWER)
            .await?;
        Ok(folder)
    }

    pub async fn create(&self, user_id: &str, mut request: CreateFolder) -> Result<Folder> {
        self.connections
            .authorize(&request.connection_id, user_id, WorkspaceRole::EDITOR)
            .await?;
        request.user_id = Some(user_id.to_owned());
        let folder = self.repository.create(request).await?;
        let params = serde_json::json!({
            "connectionId": folder.connection_id.clone(),
            "folderName": folder.title.clone(),
            "type": "FOLDER",
            "folderId": folder.id.clone(),
        });

        self.tabs
            .create(
                user_id,
                CreateTab {
                    id: folder.id.clone(),
                    connection_id: Some(folder.connection_id.clone()),
                    user_id: None,
                    title: folder.title.clone(),
                    params: serde_json::to_string(&params)?,
                },
            )
            .await?;

        Ok(folder)
    }

    pub async fn list(&self, connection_id: &str, user_id: &str) -> Result<Vec<FolderWithQueues>> {
        self.connections
            .authorize(connection_id, user_id, WorkspaceRole::VIEWER)
            .await?;
        self.repository.get_all(connection_id).await
    }

    pub async fn get(&self, folder_id: &str, user_id: &str) -> Result<Option<Folder>> {
        let Some(folder) = self.repository.get_by_id(folder_id).await? else {
            return Ok(None);
        };
        self.connections
            .authorize(&folder.connection_id, user_id, WorkspaceRole::VIEWER)
            .await?;
        Ok(Some(folder))
    }

    pub async fn update(
        &self,
        folder_id: &str,
        user_id: &str,
        request: UpdateFolder,
    ) -> Result<Folder> {
        let folder = self.folder_for_user(folder_id, user_id).await?;
        self.connections
            .authorize(&folder.connection_id, user_id, WorkspaceRole::EDITOR)
            .await?;
        if request.title.trim().is_empty() {
            bail!("Folder title cannot be empty");
        }
        let title = request.title.trim().to_owned();
        let updated_folder = self.repository.update(folder_id, &title).await?;

        if self.tabs.get(folder_id, user_id).await?.is_some() {
            self.tabs
                .update(
                    folder_id,
                    user_id,
                    UpdateTab {
                        title: Some(title),
                        ..Default::default()
                    },
                )
                .await?;
        }

        Ok(updated_folder)
    }

    pub async fn delete(&self, folder_id: &str, user_id: &str) -> Result<u64> {
        let folder = self.folder_for_user(folder_id, user_id).await?;
        self.connections
            .authorize(&folder.connection_id, user_id, WorkspaceRole::EDITOR)
            .await?;
        self.repository.delete(folder_id).await
    }

    pub async fn queues(
        &self,
        folder_id: &str,
        user_id: &str,
        redis: &RedisConnection,
    ) -> Result<Vec<ReadQueueWithCounts>> {
        let folder = self.folder_for_user(folder_id, user_id).await?;

        let queues_of_folders = self.repository.get_queues_for_folder(&folder.id).await?;

        if queues_of_folders.is_empty() {
            return Ok(vec![]);
        }

        let queue_names: Vec<&str> = queues_of_folders
            .iter()
            .map(|q| q.queue_name.as_str())
            .collect();

        let job_service =
            JobService::for_user(redis, &self.connections, &user_id, WorkspaceRole::VIEWER).await?;

        let job_counts = job_service
            .get_job_count_in_queue_service(queue_names)
            .await?;

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

    pub async fn toggle_queue(
        &self,
        folder_id: &str,
        queue_id: &str,
        user_id: &str,
    ) -> Result<bool> {
        let folder = self.folder_for_user(folder_id, user_id).await?;
        self.connections
            .authorize(&folder.connection_id, user_id, WorkspaceRole::EDITOR)
            .await?;
        self.repository
            .toggle_queue_in_folder(folder_id, queue_id)
            .await
    }

    pub async fn reorder(
        &self,
        folder_id: &str,
        queue_ids: Vec<String>,
        user_id: &str,
    ) -> Result<()> {
        let folder = self.folder_for_user(folder_id, user_id).await?;
        self.connections
            .authorize(&folder.connection_id, user_id, WorkspaceRole::EDITOR)
            .await?;
        self.repository.reorder_queues(folder_id, &queue_ids).await
    }

    pub async fn reorder_folders(
        &self,
        connection_id: &str,
        folder_ids: Vec<String>,
        user_id: &str,
    ) -> Result<()> {
        self.connections
            .authorize(connection_id, user_id, WorkspaceRole::EDITOR)
            .await?;
        self.repository
            .reorder_folders(connection_id, &folder_ids)
            .await
    }
}
