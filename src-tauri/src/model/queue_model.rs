use crate::{
    entity::queue_entity,
    model::job_model::{JobFilterType, JobStatus},
};
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadQueueModel {
    pub id: Uuid,
    pub connection_id: Uuid,
    pub queue_name: String,
    pub display_name: Option<String>,
    pub is_starred: Option<bool>,
    pub auto_refresh_rate: Option<i32>,
    pub notification_settings: Option<String>,
    pub created_at: DateTimeUtc,
}

impl From<queue_entity::Model> for ReadQueueModel {
    fn from(entity: queue_entity::Model) -> Self {
        ReadQueueModel {
            id: entity.id,
            connection_id: entity.connection_id,
            queue_name: entity.queue_name,
            display_name: entity.display_name,
            is_starred: entity.is_starred,
            auto_refresh_rate: entity.auto_refresh_rate,
            notification_settings: entity.notification_settings,
            created_at: entity.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateQueueModel {
    pub connection_id: i64,
    pub queue_name: String,
    pub display_name: Option<String>,
    pub is_starred: Option<bool>,
    pub auto_refresh_rate: Option<i32>,
    pub notification_settings: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQueueModel {
    pub display_name: Option<String>,
    pub is_starred: Option<bool>,
    pub auto_refresh_rate: Option<i32>,
    pub notification_settings: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionWithQueues {
    pub id: Uuid,
    pub connection_name: String,
    pub color: Option<String>,
    pub queues: Vec<ReadQueueModel>,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct QueueTabParams {
    connection_id: Uuid,
    queue_name: String,
    search_filters: Option<Vec<JobFilterType>>,
    state: Option<JobStatus>,
    selected_job: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct QueueJobCounts {
    pub queue_name: String,
    pub total: i64,
    pub wait: i64,
    pub active: i64,
    pub completed: i64,
    pub failed: i64,
    pub delayed: i64,
    pub paused: i64,
    pub prioritized: i64,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct QueueDetails {
    pub name: String,
    pub is_paused: bool,
    pub version: String, // From the meta hash or Redis info
    pub prefix: String,
    pub active_workers: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadQueueWithCounts {
    #[serde(flatten)]
    pub queue: ReadQueueModel,
    pub counts: QueueJobCounts,
}
