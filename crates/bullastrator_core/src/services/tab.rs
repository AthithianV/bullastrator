use anyhow::{Context, Result, bail};
use bullastrator_storage::{
    models::{CreateTab, Tab, UpdateTab, WorkspaceRole},
    repositories::TabRepository,
};

use crate::services::workspace::WorkspaceService;

#[derive(Clone)]
pub struct TabService {
    repository: TabRepository,
    workspaces: WorkspaceService,
}

impl TabService {
    pub fn new(repository: TabRepository, workspaces: WorkspaceService) -> Self {
        Self {
            repository,
            workspaces,
        }
    }

    async fn active_workspace(&self, user_id: &str) -> Result<String> {
        self.workspaces.active_id(user_id).await
    }

    async fn owned_tab(&self, tab_id: &str, user_id: &str) -> Result<Tab> {
        let tab = self
            .repository
            .get_by_id(tab_id)
            .await?
            .context("Tab not found")?;
        if tab.user_id.as_deref() != Some(user_id) {
            bail!("You do not have access to this tab");
        }
        Ok(tab)
    }

    pub async fn create(&self, user_id: &str, mut request: CreateTab) -> Result<Tab> {
        let workspace_id = self.active_workspace(user_id).await?;
        self.workspaces
            .check_permission(&workspace_id, user_id, WorkspaceRole::VIEWER)
            .await?;
        request.user_id = Some(user_id.to_owned());
        self.repository.create(&workspace_id, request).await
    }

    pub async fn get(&self, tab_id: &str, user_id: &str) -> Result<Option<Tab>> {
        let Some(tab) = self.repository.get_by_id(tab_id).await? else {
            return Ok(None);
        };
        if tab.user_id.as_deref() != Some(user_id) {
            bail!("You do not have access to this tab");
        }
        Ok(Some(tab))
    }

    pub async fn list(&self, user_id: &str) -> Result<Vec<Tab>> {
        let workspace_id = self.active_workspace(user_id).await?;
        Ok(self
            .repository
            .get_all(&workspace_id)
            .await?
            .into_iter()
            .filter(|tab| tab.user_id.as_deref() == Some(user_id))
            .collect())
    }

    pub async fn update(&self, tab_id: &str, user_id: &str, request: UpdateTab) -> Result<Tab> {
        self.owned_tab(tab_id, user_id).await?;
        self.repository.update(tab_id, request).await
    }

    pub async fn delete(&self, tab_id: &str, user_id: &str) -> Result<u64> {
        self.owned_tab(tab_id, user_id).await?;
        self.repository.delete(tab_id).await
    }

    pub async fn set_active(&self, tab_id: &str, user_id: &str) -> Result<()> {
        let tab = self.owned_tab(tab_id, user_id).await?;
        self.repository
            .set_active_tab(&tab.workspace_id, Some(tab_id.to_owned()))
            .await
    }

    pub async fn active(&self, user_id: &str) -> Result<Option<String>> {
        let workspace_id = self.active_workspace(user_id).await?;
        let Some(tab_id) = self.repository.get_active_tab(&workspace_id).await? else {
            return Ok(None);
        };
        let Some(tab) = self.repository.get_by_id(&tab_id).await? else {
            return Ok(None);
        };
        Ok((tab.user_id.as_deref() == Some(user_id)).then_some(tab_id))
    }

    pub async fn reorder(&self, user_id: &str, ordered_ids: Vec<String>) -> Result<()> {
        let workspace_id = self.active_workspace(user_id).await?;
        if ordered_ids.iter().any(|id| id.is_empty()) {
            bail!("Tab IDs cannot be empty");
        }
        for tab_id in &ordered_ids {
            let tab = self.owned_tab(tab_id, user_id).await?;
            if tab.workspace_id != workspace_id {
                bail!("Tab does not belong to the active workspace");
            }
        }
        self.repository
            .update_order(&workspace_id, &ordered_ids)
            .await
    }
}
