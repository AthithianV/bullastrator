use crate::entity::folder_queue_entity;
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadFolderQueueModel {
    pub id: Uuid,
    pub folder_id: Uuid,
    pub queue_id: Uuid,
    pub sort_order: i32,
    pub created_at: DateTimeUtc,
}

impl From<folder_queue_entity::Model> for ReadFolderQueueModel {
    fn from(entity: folder_queue_entity::Model) -> Self {
        ReadFolderQueueModel {
            id: entity.id,
            folder_id: entity.folder_id,
            queue_id: entity.queue_id,
            sort_order: entity.sort_order,
            created_at: entity.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderQueueModel {
    pub folder_id: i64,
    pub queue_id: Option<i64>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFolderQueueModel {
    pub queue_id: Option<i64>,
    pub sort_order: Option<i32>,
}
