use crate::{
    entity::{folder_entity, queue_entity},
    model::queue_model::ReadQueueModel,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadFolderModel {
    pub id: Uuid,
    pub connection_id: Uuid,
    pub title: String,
}

impl From<folder_entity::Model> for ReadFolderModel {
    fn from(entity: folder_entity::Model) -> Self {
        ReadFolderModel {
            id: entity.id,
            connection_id: entity.connection_id,
            title: entity.title,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadFolderWithQueuesModel {
    pub id: Uuid,
    pub connection_id: Uuid,
    pub title: String,
    queues: Vec<ReadQueueModel>,
}

impl From<(folder_entity::Model, Vec<queue_entity::Model>)> for ReadFolderWithQueuesModel {
    fn from(folder_with_queue: (folder_entity::Model, Vec<queue_entity::Model>)) -> Self {
        ReadFolderWithQueuesModel {
            id: folder_with_queue.0.id,
            connection_id: folder_with_queue.0.connection_id,
            title: folder_with_queue.0.title,
            queues: folder_with_queue
                .1
                .into_iter()
                .map(ReadQueueModel::from)
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderModel {
    pub connection_id: Uuid,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFolderModel {
    pub title: Option<String>,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FolderTabParams {
    connection_id: Uuid,
    folder_name: String,
    folder_id: Uuid,
}
