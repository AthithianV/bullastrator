use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct AddMemberRequest {
    pub user_id: String,
    pub role: String,
}
