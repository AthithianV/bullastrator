use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FolderQueue {
    pub id: String,
    pub folder_id: String,
    pub queue_id: String,
    pub sort_order: i32,
    pub created_at: Option<NaiveDateTime>,
}
