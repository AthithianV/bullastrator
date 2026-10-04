use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    #[serde(skip_serializing)]
    pub token_hash: String,
    pub expires_at: NaiveDateTime,
    pub revoked_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}
