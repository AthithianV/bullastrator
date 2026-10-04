use anyhow::Result;
use bullastrator_storage::{
    models::{
        CreateWorkspace, CreateWorkspaceMember, UpdateWorkspace, UpdateWorkspaceMember, Workspace,
        WorkspaceMember,
    },
    repositories::WorkspaceRepository,
};

#[derive(Clone)]
pub struct WorkspaceService {
    repository: WorkspaceRepository,
}

impl WorkspaceService {
    pub fn new(repository: WorkspaceRepository) -> Self {
        Self { repository }
    }

    #[tracing::instrument(skip(self, request), err)]
    pub async fn create(&self, user_id: &str, request: CreateWorkspace) -> Result<Workspace> {
        tracing::info!("creating workspace");
        self.repository
            .create(CreateWorkspace {
                user_id: request.user_id,
                name: request.name,
                color: request.color,
                icon: request.icon,
            })
            .await
    }

    #[tracing::instrument(skip(self), fields(workspace_id = %workspace_id), err)]
    pub async fn get(&self, workspace_id: &str, user_id: &str) -> Result<Option<Workspace>> {
        self.repository
            .check_permission(workspace_id, user_id, "VIEWER")
            .await?;
        self.repository.get_by_id(workspace_id, user_id).await
    }

    #[tracing::instrument(skip(self), fields(user_id = %user_id), err)]
    pub async fn list_for_user(&self, user_id: &str) -> Result<Vec<Workspace>> {
        self.repository.get_by_user_id(user_id).await
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn active(&self, user_id: &str) -> Result<Workspace> {
        self.repository.get_active_workspace(user_id).await
    }

    #[tracing::instrument(skip(self), fields(workspace_id = %workspace_id), err)]
    pub async fn select(&self, workspace_id: &str, user_id: &str) -> Result<Workspace> {
        self.repository
            .check_permission(workspace_id, user_id, "VIEWER")
            .await?;
        self.repository
            .set_active_workspace(workspace_id, user_id)
            .await
    }

    #[tracing::instrument(skip(self, request), fields(workspace_id = %workspace_id), err)]
    pub async fn update(
        &self,
        workspace_id: &str,
        user_id: &str,
        request: UpdateWorkspace,
    ) -> Result<Workspace> {
        self.repository
            .check_permission(workspace_id, user_id, "OWNER")
            .await?;
        self.repository
            .update(
                workspace_id,
                user_id,
                UpdateWorkspace {
                    name: request.name,
                    icon: request.icon,
                    color: request.color,
                },
            )
            .await
    }

    #[tracing::instrument(skip(self), fields(workspace_id = %workspace_id), err)]
    pub async fn delete(&self, workspace_id: &str, user_id: &str) -> Result<u64> {
        self.repository
            .check_permission(workspace_id, user_id, "OWNER")
            .await?;
        self.repository.delete(workspace_id).await
    }

    #[tracing::instrument(skip(self), fields(workspa = %workspace_id), err)]
    pub async fn list_members(
        &self,
        workspace_id: &str,
        user_id: &str,
    ) -> Result<Vec<WorkspaceMember>> {
        self.repository
            .check_permission(workspace_id, user_id, "ADMIN")
            .await?;
        self.repository.get_members(workspace_id).await
    }

    #[tracing::instrument(skip(self, request), err)]
    pub async fn add_member(
        &self,
        workspace_id: &str,
        user_id: &str,
        request: CreateWorkspaceMember,
    ) -> Result<WorkspaceMember> {
        self.repository
            .check_permission(workspace_id, user_id, "ADMIN")
            .await?;

        self.repository.add_member(request).await
    }

    #[tracing::instrument(skip(self, request), fields(user_id = %user_id, workspace_id = %workspace_id), err)]
    pub async fn update_member(
        &self,
        user_id: &str,
        workspace_id: &str,
        request: UpdateWorkspaceMember,
    ) -> Result<WorkspaceMember> {
        self.repository
            .check_permission(workspace_id, user_id, "ADMIN")
            .await?;

        self.repository
            .update_member(user_id, workspace_id, request)
            .await
    }

    #[tracing::instrument(skip(self), fields(user_id = %user_id, workspace_id = %workspace_id), err)]
    pub async fn remove_member(&self, user_id: &str, workspace_id: &str) -> Result<u64> {
        self.repository
            .check_permission(workspace_id, user_id, "ADMIN")
            .await?;

        self.repository.remove_member(user_id, workspace_id).await
    }
}
