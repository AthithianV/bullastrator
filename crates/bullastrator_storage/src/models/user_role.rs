use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserRole {
    pub id: String,
    pub user_id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub entity_name: String,
    pub role: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRole {
    pub user_id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub role: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRole {
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RoleEntityOption {
    pub id: String,
    pub name: String,
}
