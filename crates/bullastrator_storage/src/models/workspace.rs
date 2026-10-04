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
    pub role: String,
}

#[derive(Debug, Clone)]
pub struct CreateWorkspace {
    pub user_id: String,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateWorkspace {
    pub name: Option<String>,
    pub icon: Option<i32>,
    pub color: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspacePermissionError {
    #[error("You are not a member of this workspace")]
    NotMember,

    #[error("Insufficient permissions: {required} role required, but you have {actual}")]
    InsufficientRole { required: String, actual: String },

    #[error("Invalid workspace role: {0}")]
    InvalidRole(String),
}
