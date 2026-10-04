use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorkspaceMember {
    pub name: String,
    pub email: String,
    pub user_id: String,
    pub workspace_id: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkspaceMember {
    pub user_id: String,
    pub workspace_id: String,
    pub role: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWorkspaceMember {
    pub role: Option<String>,
}
