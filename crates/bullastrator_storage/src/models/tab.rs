use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Tab {
    pub id: String,
    pub workspace_id: String,
    pub connection_id: Option<String>,
    pub user_id: Option<String>,
    pub title: String,
    pub params: String,
    pub is_active: bool,
    pub is_dirty: bool,
    pub is_pinned: bool,
    pub is_preview: bool,
    pub rank: i32,

    pub connection_color: Option<String>,
    pub connection_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTab {
    pub connection_id: Option<String>,
    pub user_id: Option<String>,
    pub title: String,
    pub params: String,
    pub is_preview: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTab {
    pub title: Option<String>,
    pub params: Option<String>,
    pub rank: Option<i32>,
    pub is_active: Option<bool>,
    pub is_dirty: Option<bool>,
    pub is_pinned: Option<bool>,
    pub is_preview: Option<bool>,
}
