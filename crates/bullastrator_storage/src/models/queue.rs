use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Queue {
    pub id: String,
    pub connection_id: String,
    pub queue_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionWithQueues {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub color: Option<String>,
    pub label: Option<String>,
    pub queues: Vec<Queue>,
}
