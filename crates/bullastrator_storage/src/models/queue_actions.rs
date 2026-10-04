use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QueueAction {
    pub id: String,
    pub workspace_id: String,
    pub connection_id: String,
    pub user_id: String,
    pub action: String,
    pub metadata: Option<String>,
    pub created_at: NaiveDateTime,
}
