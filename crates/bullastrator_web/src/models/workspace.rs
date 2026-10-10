use bullastrator_storage::models::WorkspaceRole;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct RequiredWorkspaceRole {
    pub workspace_role: WorkspaceRole,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AddMemberRequest {
    pub user_id: String,
    pub role: WorkspaceRole,
}
