use crate::entity::{
    SettingsActiveModel, SettingsColumn, SettingsEntity, WorkspaceActiveModel, WorkspaceColumn,
    WorkspaceEntity,
};
use crate::model::workspace_model::{
    CreateWorkspaceModel, ReadWorkspaceModel, UpdateWorkspaceModel,
};
use anyhow::{Context, Result, bail};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DeleteResult, EntityTrait, QueryOrder, Set, prelude::*,
    sea_query,
};

pub struct WorkspaceRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> WorkspaceRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    // --- CREATE ---
    pub async fn create(&self, data: CreateWorkspaceModel) -> Result<ReadWorkspaceModel> {
        let id = Uuid::new_v4();

        let new_workspace = WorkspaceActiveModel {
            id: Set(id),
            user_id: Set(data.user_id.clone()),
            active_tab_id: Default::default(),
            name: Set(data.name.clone()),
            icon: Set(data.icon.clone()),
            plan: Set(data.plan.clone()),
            role: Set(data.role.clone()),
            max_connections: Set(data.max_connections),
            color: Set(data.color.clone()),
            last_accessed_at: Set(data.last_accessed_at),
            created_at: Default::default(),
            is_guest_mode: Set(false),
            is_primary: Set(false),
        };

        WorkspaceEntity::insert_many(vec![new_workspace])
            .exec_without_returning(self.db)
            .await?;

        Ok(ReadWorkspaceModel {
            id,
            user_id: data.user_id.clone(),
            active_tab_id: Default::default(),
            name: data.name,
            icon: data.icon,
            color: data.color,
            plan: data.plan,
            role: data.role,
            max_connections: data.max_connections,
            last_accessed_at: data.last_accessed_at,
            created_at: Utc::now(),
            is_guest_mode: false,
            is_primary: false,
        })
    }

    pub async fn get_by_user_id(&self, user_id: &str) -> Result<Vec<ReadWorkspaceModel>> {
        let workspaces = WorkspaceEntity::find()
            .filter(WorkspaceColumn::UserId.eq(user_id))
            .all(self.db)
            .await
            .context("Failed to get Workspaces by user_id")?;

        Ok(workspaces
            .into_iter()
            .map(ReadWorkspaceModel::from)
            .collect())
    }

    // --- GET ALL ---
    pub async fn get_all(&self) -> Result<Vec<ReadWorkspaceModel>> {
        let workspaces = WorkspaceEntity::find()
            .filter(WorkspaceColumn::IsGuestMode.eq(false))
            .all(self.db)
            .await
            .context("Failed to get all Workspaces")?;

        Ok(workspaces
            .into_iter()
            .map(ReadWorkspaceModel::from)
            .collect())
    }

    pub async fn get_active_workspace(&self) -> Result<ReadWorkspaceModel> {
        // 1. Try to get active_workspace_id from settings table
        let active_ws_setting = SettingsEntity::find_by_id("active_workspace_id".to_string())
            .one(self.db)
            .await
            .context("Database error while fetching settings")?;

        let target_id: Option<Uuid> = active_ws_setting.and_then(|s| s.value.parse::<Uuid>().ok());

        // 2. If not in settings, find the one with the greatest last_accessed_at
        if target_id.is_none() {
            let latest_ws = WorkspaceEntity::find()
                .order_by_desc(WorkspaceColumn::LastAccessedAt)
                .one(self.db)
                .await
                .context("Failed to fetch workspaces for fallback")?
                .ok_or_else(|| anyhow::anyhow!("No workspaces found in the database"))?;

            let id = latest_ws.id;

            // Save this ID to settings so it's available next time
            let new_setting = SettingsActiveModel {
                key: Set("active_workspace_id".to_string()),
                value: Set(id.to_string()),
            };

            // Upsert the setting
            SettingsEntity::insert(new_setting)
                .on_conflict(
                    sea_query::OnConflict::column(SettingsColumn::Key)
                        .update_column(SettingsColumn::Value)
                        .to_owned(),
                )
                .exec(self.db)
                .await
                .context("Failed to update active_workspace_id in settings")?;

            return Ok(ReadWorkspaceModel::from(latest_ws));
        }

        // 3. If we have a target_id (from step 1), fetch that specific workspace
        let workspace = WorkspaceEntity::find_by_id(target_id.unwrap())
            .one(self.db)
            .await
            .context("Database error while fetching specific workspace")?
            .ok_or_else(|| {
                anyhow::anyhow!("Active workspace ID {} not found", target_id.unwrap())
            })?;

        Ok(ReadWorkspaceModel::from(workspace))
    }

    pub async fn set_active_workspace(&self, workspace_id: Uuid) -> Result<()> {
        let new_setting = SettingsActiveModel {
            key: Set("active_workspace_id".to_string()),
            value: Set(workspace_id.to_string()),
        };

        // Upsert the setting
        SettingsEntity::insert(new_setting)
            .on_conflict(
                sea_query::OnConflict::column(SettingsColumn::Key)
                    .update_column(SettingsColumn::Value)
                    .to_owned(),
            )
            .exec(self.db)
            .await
            .context("Failed to update active_workspace_id in settings")?;

        Ok(())
    }

    pub async fn go_guest_mode(&self) -> Result<ReadWorkspaceModel> {
        // 3. If we have a target_id (from step 1), fetch that specific workspace
        let workspace = WorkspaceEntity::find()
            .filter(WorkspaceColumn::IsGuestMode.eq(true))
            .one(self.db)
            .await
            .context("Can't find workspace with id")?
            .ok_or_else(|| anyhow::anyhow!("Guest Mode workspace not found"))?;

        self.set_active_workspace(workspace.id).await?;

        Ok(ReadWorkspaceModel::from(workspace))
    }

    pub async fn get_by_id(&self, workspace_id: Uuid) -> Result<ReadWorkspaceModel> {
        let result = WorkspaceEntity::find_by_id(workspace_id)
            .one(self.db)
            .await
            .context("Can't find workspace with id")?
            .ok_or_else(|| anyhow::anyhow!("Workspace with Id {} not found", workspace_id))?;

        Ok(ReadWorkspaceModel::from(result))
    }

    // --- UPDATE ---
    pub async fn update(&self, id: Uuid, data: UpdateWorkspaceModel) -> Result<ReadWorkspaceModel> {
        let workspace = WorkspaceEntity::find_by_id(id)
            .one(self.db)
            .await
            .context("Database error finding workspace")?
            .ok_or_else(|| anyhow::anyhow!("Workspace ID {} not found", id))?;

        let mut active: WorkspaceActiveModel = workspace.into();

        if let Some(name) = data.name {
            active.name = Set(name);
        }
        if let Some(icon) = data.icon {
            active.icon = Set(Some(icon));
        }
        if let Some(color) = data.color {
            active.color = Set(Some(color));
        }
        if let Some(last_accessed_at) = data.last_accessed_at {
            active.last_accessed_at = Set(last_accessed_at);
        }
        if let Some(is_guest_mode) = data.is_guest_mode {
            active.is_guest_mode = Set(is_guest_mode);
        }
        if let Some(is_primary) = data.is_primary {
            active.is_primary = Set(is_primary);
        }

        let updated = active
            .update(self.db)
            .await
            .context("Failed to update workspace")?;

        Ok(ReadWorkspaceModel::from(updated))
    }

    // --- DELETE ---
    pub async fn delete(&self, id: Uuid) -> Result<DeleteResult> {
        let res = WorkspaceEntity::delete_by_id(id)
            .exec(self.db)
            .await
            .context("Failed to delete workspace")?;

        if res.rows_affected == 0 {
            bail!("Workspace ID {} not found", id);
        }

        Ok(res)
    }

    pub async fn upsert(&self, data: CreateWorkspaceModel, id: Uuid) -> Result<ReadWorkspaceModel> {
        let workspace = WorkspaceActiveModel {
            id: Set(id),
            user_id: Set(data.user_id.clone()),
            name: Set(data.name.clone()),
            plan: Set(data.plan.clone()),
            role: Set(data.role.clone()),
            max_connections: Set(data.max_connections),
            icon: Set(data.icon.clone()),
            color: Set(data.color.clone()),
            last_accessed_at: Set(data.last_accessed_at),
            is_guest_mode: Set(false),
            is_primary: Set(false),
            created_at: Set(Utc::now()),
            active_tab_id: Set(None),
        };

        WorkspaceEntity::insert(workspace)
            .on_conflict(
                sea_query::OnConflict::column(WorkspaceColumn::Id)
                    .update_columns([
                        WorkspaceColumn::Name,
                        WorkspaceColumn::Plan,
                        WorkspaceColumn::Role,
                        WorkspaceColumn::MaxConnections,
                        WorkspaceColumn::UserId,
                        WorkspaceColumn::IsPrimary,
                    ])
                    .to_owned(),
            )
            .exec_without_returning(self.db)
            .await
            .context("Failed to upsert workspace")?;

        self.get_by_id(id).await
    }

    pub async fn select_workspace(&self, workspace_id: i64) -> Result<ReadWorkspaceModel> {
        let workspace = WorkspaceEntity::find()
            .filter(WorkspaceColumn::Id.eq(workspace_id))
            .one(self.db)
            .await
            .context("Failed to fetch workspace with id")?
            .ok_or_else(|| anyhow::anyhow!("Workspace with id {} not found", workspace_id))?;

        let mut active: WorkspaceActiveModel = workspace.clone().into();

        active.last_accessed_at = Set(Utc::now());

        let new_setting = SettingsActiveModel {
            key: Set("active_workspace_id".to_string()),
            value: Set(workspace_id.to_string()),
        };

        // Upsert the setting
        SettingsEntity::insert(new_setting)
            .on_conflict(
                sea_query::OnConflict::column(SettingsColumn::Key)
                    .update_column(SettingsColumn::Value)
                    .to_owned(),
            )
            .exec(self.db)
            .await
            .context("Failed to update active_workspace_id in settings")?;

        Ok(ReadWorkspaceModel::from(workspace))
    }
}
