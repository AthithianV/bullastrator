use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Connection {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub password: Option<String>,
    pub username: Option<String>,
    pub db: Option<i32>,
    pub last_synced_at: Option<NaiveDateTime>,
    pub bullmq_prefix: Option<String>,
    pub is_tls_enabled: Option<bool>,
    pub color: Option<String>,
    pub label: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct CreateConnection {
    pub name: String,
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub db: Option<i32>,
    pub bullmq_prefix: Option<String>,
    pub color: Option<String>,
    pub label: Option<String>,
    pub is_tls_enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateConnection {
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub db: Option<i32>,
    pub bullmq_prefix: Option<String>,
    pub color: Option<String>,
    pub label: Option<String>,
    pub is_tls_enabled: Option<bool>,
}
