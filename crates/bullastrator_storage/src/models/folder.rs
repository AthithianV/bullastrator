use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: String,
    pub connection_id: String,
    pub user_id: Option<String>,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct CreateFolder {
    pub connection_id: String,
    pub user_id: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FolderWithQueues {
    pub folder: Folder,
    pub queues: Vec<super::Queue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct FolderAndQueues {
    pub folder_id: String,
    pub folder_connection_id: String,
    pub folder_user_id: Option<String>,
    pub folder_title: String,
    pub queue_id: String,
    pub queue_connection_id: String,
    pub queue_name: String,
}
