use anyhow::Result;
use bullastrator_storage::{
    models::{
        CreateWorkspace, CreateWorkspaceMember, UpdateWorkspace, UpdateWorkspaceMember, Workspace,
        WorkspaceMember,
    },
    repositories::WorkspaceRepository,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct WorkspaceService {
    repository: WorkspaceRepository,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub user_id: Option<String>,
    pub name: String,
    pub color: Option<String>,
    pub icon: Option<i32>,
    pub plan: Option<String>,
    pub role: Option<String>,
    pub max_connections: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: Option<String>,
    pub color: Option<String>,
    pub icon: Option<i32>,
    pub last_accessed_at: Option<chrono::NaiveDateTime>,
    pub is_guest_mode: Option<bool>,
    pub is_primary: Option<bool>,
}

impl WorkspaceService {
    pub fn new(repository: WorkspaceRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, request: CreateWorkspaceRequest) -> Result<Workspace> {
        self.repository
            .create(CreateWorkspace {
                user_id: request.user_id,
                name: request.name,
                color: request.color,
                icon: request.icon,
                plan: request.plan.unwrap_or_else(|| "FREE".into()),
                role: request.role.unwrap_or_else(|| "OWNER".into()),
                max_connections: request.max_connections.unwrap_or(1),
                last_accessed_at: Utc::now().naive_utc(),
            })
            .await
    }

    pub async fn get(&self, id: &str) -> Result<Option<Workspace>> {
        self.repository.get_by_id(id).await
    }
    pub async fn list_for_user(&self, user_id: &str) -> Result<Vec<Workspace>> {
        self.repository.get_by_user_id(user_id).await
    }
    pub async fn active(&self) -> Result<Workspace> {
        self.repository.get_active_workspace().await
    }
    pub async fn select(&self, id: &str) -> Result<Workspace> {
        self.repository.select_workspace(id).await
    }

    pub async fn update(&self, id: &str, request: UpdateWorkspaceRequest) -> Result<Workspace> {
        self.repository
            .update(
                id,
                UpdateWorkspace {
                    name: request.name,
                    icon: request.icon,
                    color: request.color,
                    last_accessed_at: request.last_accessed_at,
                    is_guest_mode: request.is_guest_mode,
                    is_primary: request.is_primary,
                },
            )
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        self.repository.delete(id).await
    }

    pub async fn list_members(&self, connection_id: &str) -> Result<Vec<WorkspaceMember>> {
        self.repository.get_members(connection_id).await
    }

    pub async fn add_member(&self, request: CreateWorkspaceMember) -> Result<WorkspaceMember> {
        self.repository.add_member(request).await
    }

    pub async fn update_member(
        &self,
        user_id: &str,
        connection_id: &str,
        request: UpdateWorkspaceMember,
    ) -> Result<WorkspaceMember> {
        self.repository
            .update_member(user_id, connection_id, request)
            .await
    }

    pub async fn remove_member(&self, user_id: &str, connection_id: &str) -> Result<u64> {
        self.repository.remove_member(user_id, connection_id).await
    }
}
