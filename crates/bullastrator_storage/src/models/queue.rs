use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Queue {
    pub id: String,
    pub connection_id: String,
    pub queue_name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateQueue {
    pub display_name: Option<String>,
    pub is_starred: Option<bool>,
    pub auto_refresh_rate: Option<i32>,
    pub notification_settings: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConnectionWithQueues {
    pub connection: super::Connection,
    pub queues: Vec<Queue>,
}
