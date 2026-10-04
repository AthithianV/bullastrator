use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Settings {
    pub key: String,
    pub user_id: Option<String>,
    pub value: Option<String>,
}
