use anyhow::{Context, Result, bail};
use bullastrator_storage::{
    models::{CreateTab, Tab, UpdateTab},
    repositories::{TabRepository, WorkspaceRepository},
};
use uuid::Uuid;

#[derive(Clone)]
pub struct TabService {
    repository: TabRepository,
    workspaces: WorkspaceRepository,
}

impl TabService {
    pub fn new(repository: TabRepository, workspaces: WorkspaceRepository) -> Self {
        Self {
            repository,
            workspaces,
        }
    }

    async fn active_workspace(&self, user_id: &str) -> Result<String> {
        Ok(self.workspaces.get_active_workspace(user_id).await?.id)
    }

    async fn authorized_tab_workspace(&self, tab_id: &str, user_id: &str) -> Result<String> {
        let tab = self
            .repository
            .get_by_id(tab_id)
            .await?
            .context("Tab not found")?;
        self.workspaces
            .check_permission(&tab.workspace_id, user_id, "VIEWER")
            .await?;
        Ok(tab.workspace_id)
    }

    pub async fn create(&self, user_id: &str, mut request: CreateTab) -> Result<Tab> {
        let workspace_id = self.active_workspace(user_id).await?;
        self.workspaces
            .check_permission(&workspace_id, user_id, "EDITOR")
            .await?;
        request.user_id = Some(user_id.to_owned());
        self.repository
            .create(&Uuid::new_v4().to_string(), &workspace_id, request)
            .await
    }

    pub async fn get(&self, tab_id: &str, user_id: &str) -> Result<Option<Tab>> {
        let Some(tab) = self.repository.get_by_id(tab_id).await? else {
            return Ok(None);
        };
        self.workspaces
            .check_permission(&tab.workspace_id, user_id, "VIEWER")
            .await?;
        Ok(Some(tab))
    }

    pub async fn list(&self, user_id: &str) -> Result<Vec<Tab>> {
        let workspace_id = self.active_workspace(user_id).await?;
        self.workspaces
            .check_permission(&workspace_id, user_id, "VIEWER")
            .await?;
        self.repository.get_all(&workspace_id).await
    }

    pub async fn update(&self, tab_id: &str, user_id: &str, request: UpdateTab) -> Result<Tab> {
        let workspace_id = self.authorized_tab_workspace(tab_id, user_id).await?;
        self.workspaces
            .check_permission(&workspace_id, user_id, "EDITOR")
            .await?;
        self.repository.update(tab_id, request).await
    }

    pub async fn delete(&self, tab_id: &str, user_id: &str) -> Result<u64> {
        let workspace_id = self.authorized_tab_workspace(tab_id, user_id).await?;
        self.workspaces
            .check_permission(&workspace_id, user_id, "EDITOR")
            .await?;
        self.repository.delete(tab_id).await
    }

    pub async fn set_active(&self, tab_id: &str, user_id: &str) -> Result<()> {
        let workspace_id = self.authorized_tab_workspace(tab_id, user_id).await?;
        self.workspaces
            .check_permission(&workspace_id, user_id, "EDITOR")
            .await?;
        self.repository
            .set_active_tab(&workspace_id, Some(tab_id.to_owned()))
            .await
    }

    pub async fn active(&self, user_id: &str) -> Result<Option<String>> {
        let workspace_id = self.active_workspace(user_id).await?;
        self.workspaces
            .check_permission(&workspace_id, user_id, "VIEWER")
            .await?;
        self.repository.get_active_tab(&workspace_id).await
    }

    pub async fn reorder(&self, user_id: &str, ordered_ids: Vec<String>) -> Result<()> {
        let workspace_id = self.active_workspace(user_id).await?;
        self.workspaces
            .check_permission(&workspace_id, user_id, "EDITOR")
            .await?;
        if ordered_ids.iter().any(|id| id.is_empty()) {
            bail!("Tab IDs cannot be empty");
        }
        self.repository
            .update_order(&workspace_id, &ordered_ids)
            .await
    }
}
