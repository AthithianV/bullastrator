use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub last_synced_at: Option<NaiveDateTime>,
    pub color: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDetails {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    pub db: Option<i32>,
    pub is_tls_enabled: bool,
    pub bullmq_prefix: Option<String>,
    pub color: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConnectionCredentials {
    pub id: String,
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    pub db: Option<i32>,
    pub bullmq_prefix: Option<String>,
    pub is_tls_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InsecureConnectionCredentials {
    pub id: String,
    pub host: String,
    pub port: i32,
    pub password: Option<String>,
    pub username: Option<String>,
    pub db: Option<i32>,
    pub bullmq_prefix: Option<String>,
    pub is_tls_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
