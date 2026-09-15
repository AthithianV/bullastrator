use crate::entity::workspace_entity;
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadWorkspaceModel {
    pub id: Uuid,
    pub user_id: Option<String>,
    pub name: String,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub active_tab_id: Option<String>,
    pub last_accessed_at: DateTimeUtc,
    pub plan: String,
    pub role: String,
    pub max_connections: u32,
    pub created_at: DateTimeUtc,
    pub is_guest_mode: bool,
    pub is_primary: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiWorkspaceModel {
    pub id: Uuid,
    pub name: String,
    pub owner_id: String,
    pub is_primary: bool,
    pub role: String,
    #[serde(rename = "maxConnection")]
    pub max_connection: u32,
}

impl From<workspace_entity::Model> for ReadWorkspaceModel {
    fn from(entity: workspace_entity::Model) -> Self {
        ReadWorkspaceModel {
            id: entity.id,
            user_id: entity.user_id,
            name: entity.name,
            icon: entity.icon,
            color: entity.color,
            active_tab_id: entity.active_tab_id,
            created_at: entity.created_at,
            plan: entity.plan,
            role: entity.role,
            max_connections: entity.max_connections,
            last_accessed_at: entity.last_accessed_at,
            is_guest_mode: entity.is_guest_mode,
            is_primary: entity.is_primary,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkspaceModel {
    pub name: String,
    pub plan: String,
    pub role: String,
    pub max_connections: u32,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub user_id: Option<String>,
    pub last_accessed_at: DateTimeUtc,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkspaceModel {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub last_accessed_at: Option<DateTimeUtc>,
    pub is_guest_mode: Option<bool>,
    pub is_primary: Option<bool>,
}
