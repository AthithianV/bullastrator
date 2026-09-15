use crate::entity::{
    ConnectionEntity, TabActiveModel, TabColumn, TabEntity, WorkspaceActiveModel, WorkspaceEntity,
};
use crate::model::tab_model::{CreateTabModel, ReadTabModel, UpdateTabModel};
use anyhow::{Context, Result, bail};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DeleteResult, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
    Set, TransactionTrait, prelude::*,
};

pub struct TabRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> TabRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    // --- CREATE ---
    pub async fn create(
        &self,
        id: String,
        workspace_id: Uuid,
        data: CreateTabModel,
    ) -> Result<ReadTabModel> {
        let existing_tab = TabEntity::find()
            .filter(TabColumn::Id.eq(id.clone()))
            .one(self.db)
            .await?;

        if let Some(tab) = existing_tab {
            self.set_active_tab(workspace_id, Some(tab.id.clone()))
                .await?;
            return Ok(ReadTabModel::from(tab));
        }

        let max_rank_query = TabEntity::find()
            .filter(TabColumn::WorkspaceId.eq(workspace_id))
            .select_only()
            .column_as(TabColumn::Rank.max(), "max_rank")
            .into_tuple::<Option<i32>>()
            .one(self.db)
            .await?;

        // If None (no tabs exist), start at 0. Otherwise, max + 1.
        let next_rank = max_rank_query.unwrap_or(Some(0)).unwrap_or(0) + 1;

        let new_tab = TabActiveModel {
            id: Set(id),
            workspace_id: Set(workspace_id),
            connection_id: Set(data.connection_id),
            title: Set(data.title),
            params: Set(data.params),

            is_preview: Set(data.is_preview.unwrap_or(false)),
            rank: Set(next_rank),

            is_active: Set(false),
            is_dirty: Set(false),
            is_pinned: Set(false),

            created_at: Default::default(),
            updated_at: Default::default(),
        };

        let inserted_row = new_tab
            .insert(self.db)
            .await
            .with_context(|| "Failed to create new Tab")?;

        self.set_active_tab(workspace_id, Some(inserted_row.id.clone()))
            .await?;

        Ok(ReadTabModel::from(inserted_row))
    }

    // --- GET ALL ---
    pub async fn get_all(&self, workspace_id: Uuid) -> Result<Vec<ReadTabModel>> {
        let tab_vec = TabEntity::find()
            .find_also_related(ConnectionEntity)
            .filter(TabColumn::WorkspaceId.eq(workspace_id))
            .order_by_asc(TabColumn::Rank) // Important: Maintain visual order
            .all(self.db)
            .await
            .with_context(|| "Failed to fetch tabs from database")?;

        Ok(tab_vec
            .into_iter()
            .map(|(tab, conn)| ReadTabModel::from_with_connection(tab, conn))
            .collect())
    }

    // --- GET BY ID ---
    pub async fn get_by_id(&self, id: String) -> Result<Option<ReadTabModel>> {
        let tab_opt = TabEntity::find_by_id(id.clone())
            .find_also_related(ConnectionEntity)
            .one(self.db)
            .await
            .with_context(|| format!("Failed to get tab with ID: {}", id))?;

        Ok(tab_opt.map(|(tab, conn)| ReadTabModel::from_with_connection(tab, conn)))
    }

    // --- UPDATE ---
    pub async fn update(&self, id: String, data: UpdateTabModel) -> Result<ReadTabModel> {
        // 1. Find existing record
        let tab_model = TabEntity::find_by_id(id.clone())
            .one(self.db)
            .await
            .with_context(|| format!("Failed to find tab with ID: {} for Update", id))?
            .ok_or_else(|| anyhow::anyhow!("Tab with ID {} not found", id))?;

        let workspace_id = tab_model.workspace_id;
        // 2. Convert to ActiveModel
        let mut active: TabActiveModel = tab_model.into();

        // 3. Apply Updates (Only for Some fields)
        if let Some(title) = data.title {
            active.title = Set(title);
        }
        if let Some(params) = data.params {
            active.params = Set(params);
        }
        if let Some(rank) = data.rank {
            active.rank = Set(rank);
        }
        // State Flags
        if let Some(is_active) = data.is_active {
            active.is_active = Set(is_active);
        }
        if let Some(is_dirty) = data.is_dirty {
            active.is_dirty = Set(is_dirty);
        }
        if let Some(is_pinned) = data.is_pinned {
            active.is_pinned = Set(is_pinned);
        }
        if let Some(is_preview) = data.is_preview {
            active.is_preview = Set(is_preview);
        }

        // 4. Save
        let updated = active
            .update(self.db)
            .await
            .context("Failed to save updated tab")?;

        self.set_active_tab(workspace_id, Some(id)).await?;

        Ok(ReadTabModel::from(updated))
    }

    // --- DELETE ---
    pub async fn delete(&self, id: String) -> Result<DeleteResult> {
        let res = TabEntity::delete_by_id(id.clone())
            .exec(self.db)
            .await
            .with_context(|| format!("Failed to delete tab with ID {}", id))?;

        if res.rows_affected == 0 {
            bail!("Tab with ID {} not found", id);
        }

        Ok(res)
    }

    // --- REORDER (Drag & Drop) ---
    pub async fn update_order(&self, workspace_id: Uuid, ordered_ids: Vec<String>) -> Result<()> {
        // 1. Start a Transaction
        let txn = self
            .db
            .begin()
            .await
            .context("Failed to begin transaction")?;

        // 2. Loop through the IDs and update their Rank based on the index
        for (index, tab_id) in ordered_ids.iter().enumerate() {
            TabEntity::update_many()
                .col_expr(TabColumn::Rank, Expr::value(index as i32))
                .filter(TabColumn::Id.eq(tab_id))
                // Safety: Ensure we only touch tabs in the requested workspace
                .filter(TabColumn::WorkspaceId.eq(workspace_id))
                .exec(&txn)
                .await
                .with_context(|| format!("Failed to update rank for tab {}", tab_id))?;
        }

        // 3. Commit
        txn.commit().await.context("Failed to commit transaction")?;

        Ok(())
    }

    pub async fn set_active_tab(&self, workspace_id: Uuid, tab_id: Option<String>) -> Result<()> {
        let txn = self
            .db
            .begin()
            .await
            .context("Failed to begin transaction")?;

        // 1. Find the workspace
        let workspace = WorkspaceEntity::find_by_id(workspace_id)
            .one(&txn)
            .await
            .context("Failed to find workspace")?
            .ok_or_else(|| anyhow::anyhow!("Workspace with ID {} not found", workspace_id))?;

        // 2. Update the workspace's active_tab_id
        let mut active: WorkspaceActiveModel = workspace.into();
        active.active_tab_id = Set(tab_id);

        let _ = active
            .update(&txn)
            .await
            .context("Failed to update workspace")?;

        // 3. Commit
        txn.commit().await.context("Failed to commit transaction")?;

        Ok(())
    }

    pub async fn get_active_tab(&self, workspace_id: Uuid) -> Result<Option<String>> {
        let workspace = WorkspaceEntity::find_by_id(workspace_id)
            .one(self.db)
            .await
            .context("Failed to find workspace")?
            .ok_or_else(|| anyhow::anyhow!("Workspace with ID {} not found", workspace_id))?;

        Ok(workspace.active_tab_id)
    }
}
