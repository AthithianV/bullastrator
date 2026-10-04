use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Workspace {
    pub id: String,
    pub user_id: Option<String>,
    pub name: String,
    pub color: Option<String>,
    pub active_tab_id: Option<String>,
    pub icon: Option<i32>,
    pub last_accessed_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub plan: String,
    pub role: String,
    pub max_connections: i32,
    pub is_guest_mode: bool,
    pub is_primary: bool,
}

#[derive(Debug, Clone)]
pub struct CreateWorkspace {
    pub user_id: Option<String>,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<i32>,
    pub plan: String,
    pub role: String,
    pub max_connections: i32,
    pub last_accessed_at: NaiveDateTime,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateWorkspace {
    pub name: Option<String>,
    pub icon: Option<i32>,
    pub color: Option<String>,
    pub last_accessed_at: Option<NaiveDateTime>,
    pub is_guest_mode: Option<bool>,
    pub is_primary: Option<bool>,
}
