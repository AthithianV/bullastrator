use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::str::FromStr;
use strum_macros::Display;

use crate::models::workspace::WorkspacePermissionError;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceMember {
    pub name: String,
    pub email: String,
    pub user_id: String,
    pub workspace_id: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkspaceMember {
    pub user_id: String,
    pub workspace_id: String,
    pub role: WorkspaceRole,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkspaceMember {
    pub role: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum WorkspaceRole {
    #[strum(to_string = "OWNER")]
    OWNER,

    #[strum(to_string = "ADMIN")]
    ADMIN,

    #[strum(to_string = "EDITOR")]
    EDITOR,

    #[strum(to_string = "VIEWER")]
    VIEWER,
}

impl WorkspaceRole {
    pub fn from_string(role: &str) -> Result<Self, WorkspacePermissionError> {
        role.parse()
    }
}

impl FromStr for WorkspaceRole {
    type Err = WorkspacePermissionError;

    fn from_str(role: &str) -> Result<Self, WorkspacePermissionError> {
        match role.trim().to_ascii_lowercase().as_str() {
            "owner" => Some(Self::OWNER),
            "admin" => Some(Self::ADMIN),
            "editor" => Some(Self::EDITOR),
            "viewer" => Some(Self::VIEWER),
            _ => None,
        }
        .ok_or_else(|| WorkspacePermissionError::InvalidRole(role.to_string()))
    }
}
