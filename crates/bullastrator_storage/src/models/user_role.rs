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
    pub workspace_id: Option<String>,
    pub workspace_name: Option<String>,
    pub connection_id: Option<String>,
    pub connection_name: Option<String>,
    pub folder_scope: Option<String>,
    pub queue_scope: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRole {
    pub user_id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub role: String,
    #[serde(default = "default_selected_scope")]
    pub folder_scope: String,
    #[serde(default = "default_selected_scope")]
    pub queue_scope: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRole {
    pub role: String,
    pub folder_scope: Option<String>,
    pub queue_scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RoleEntityOption {
    pub id: String,
    pub name: String,
    pub workspace_id: Option<String>,
    pub workspace_name: Option<String>,
    pub connection_id: Option<String>,
    pub connection_name: Option<String>,
    pub is_inherited: bool,
    pub inherited_role: Option<String>,
}

fn default_selected_scope() -> String {
    "SELECTED".to_owned()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub next_offset: Option<usize>,
    pub has_more: bool,
}
