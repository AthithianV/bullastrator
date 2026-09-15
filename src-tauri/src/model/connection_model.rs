use crate::entity::connection_entity;
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LessSecureConnectionModel {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub db: i32,
    pub is_tls_enabled: bool,
    pub last_synced_at: Option<DateTimeUtc>,
    pub bullmq_prefix: String,
    pub created_at: DateTimeUtc,
    pub color: Option<String>,
    pub label: Option<String>,
}

impl From<connection_entity::Model> for LessSecureConnectionModel {
    fn from(entity: connection_entity::Model) -> Self {
        LessSecureConnectionModel {
            id: entity.id,
            workspace_id: entity.workspace_id,
            name: entity.name,
            host: entity.host,
            port: entity.port,
            username: entity.username,
            password: entity.password,
            db: entity.db,
            is_tls_enabled: entity.is_tls_enabled,
            created_at: entity.created_at,
            last_synced_at: entity.last_synced_at,
            bullmq_prefix: entity.bullmq_prefix,
            color: entity.color,
            label: entity.label,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadConnectionModel {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    pub db: i32,
    pub is_tls_enabled: bool,
    pub last_synced_at: Option<DateTimeUtc>,
    pub bullmq_prefix: String,
    pub color: Option<String>,
    pub label: Option<String>,
    pub created_at: DateTimeUtc,
}

impl From<connection_entity::Model> for ReadConnectionModel {
    fn from(entity: connection_entity::Model) -> Self {
        ReadConnectionModel {
            id: entity.id,
            workspace_id: entity.workspace_id,
            name: entity.name,
            host: entity.host,
            port: entity.port,
            username: entity.username,
            db: entity.db,
            is_tls_enabled: entity.is_tls_enabled,
            created_at: entity.created_at,
            last_synced_at: entity.last_synced_at,
            bullmq_prefix: entity.bullmq_prefix,
            color: entity.color,
            label: entity.label,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateConnectionModel {
    pub name: String,
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>, // ⚠️ encrypt this with stronghold/keychain
    pub db: Option<i32>,
    pub bullmq_prefix: Option<String>,
    pub color: Option<String>,
    pub label: Option<String>,
    pub is_tls_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConnectionModel {
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub db: Option<i32>,
    pub bullmq_prefix: Option<String>,
    pub is_tls_enabled: Option<bool>,

    pub color: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionHealth {
    pub id: Uuid,
    pub is_active: bool,
}
