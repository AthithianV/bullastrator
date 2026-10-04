use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserAccessConnection {
    pub id: String,
    pub user_id: String,
    pub connection_id: String,
    pub folder_id: Option<String>,
    pub queue_id: Option<String>,
    pub role: String,
    pub created_at: NaiveDateTime,
}
