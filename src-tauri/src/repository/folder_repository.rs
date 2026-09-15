use crate::{
    entity::{
        FolderActiveModel, FolderColumn, FolderEntity, FolderQueueActiveModel, FolderQueueColumn,
        FolderQueueEntity, QueueEntity, TabEntity,
    },
    model::{
        folder_model::{CreateFolderModel, ReadFolderModel, ReadFolderWithQueuesModel},
        queue_model::ReadQueueModel,
    },
};
use anyhow::{Context, Result, bail};
use migration::Expr;
use sea_orm::*;
use uuid::Uuid;

pub struct FolderRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> FolderRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    // --- CREATE ---
    pub async fn create(&self, data: &CreateFolderModel) -> Result<ReadFolderModel> {
        let base_title = data.title.clone().unwrap_or_else(|| "Untitled".to_string());
        let mut unique_title = base_title.clone();
        let mut counter = 1;

        // 1. Loop until we find a title that doesn't exist for this connection
        while FolderEntity::find()
            .filter(FolderColumn::Title.eq(unique_title.clone()))
            .filter(FolderColumn::ConnectionId.eq(data.connection_id))
            .one(self.db)
            .await
            .with_context(|| "Failed to check folder title uniqueness")?
            .is_some()
        {
            // If "Untitled" exists, try "Untitled (1)", then "Untitled (2)", etc.
            unique_title = format!("{} ({})", base_title, counter);
            counter += 1;
        }

        let id = Uuid::new_v4();
        let new_folder = FolderActiveModel {
            id: Set(id),
            connection_id: Set(data.connection_id),
            title: Set(unique_title), // Use the generated unique title
            ..Default::default()
        };

        let _ = FolderEntity::insert(new_folder)
            .exec_without_returning(self.db)
            .await
            .with_context(|| format!("Failed to create new Connection"))?;

        let inserted_row_option = FolderEntity::find_by_id(id).one(self.db).await?;

        let inserted_row = match inserted_row_option {
            Some(row) => row,
            None => bail!("Could not insert a row"),
        };

        let result = ReadFolderModel::from(inserted_row);
        Ok(result)
    }

    pub async fn get_all(&self, connection_id: Uuid) -> Result<Vec<ReadFolderWithQueuesModel>> {
        let folders = FolderEntity::find()
            .filter(FolderColumn::ConnectionId.eq(connection_id))
            .find_with_related(QueueEntity)
            .order_by_asc(FolderColumn::CreatedAt)
            .all(self.db)
            .await?;
        Ok(folders
            .into_iter()
            .map(ReadFolderWithQueuesModel::from)
            .collect())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<ReadFolderModel>> {
        let folder = FolderEntity::find_by_id(id).one(self.db).await?;
        Ok(folder.map(ReadFolderModel::from))
    }

    pub async fn update(&self, id: Uuid, title: String) -> Result<ReadFolderModel> {
        let folder = FolderActiveModel {
            id: Set(id),
            title: Set(title.clone()),
            ..Default::default()
        };

        let updated_folder = folder.update(self.db).await?;
        Ok(ReadFolderModel::from(updated_folder))
    }

    pub async fn delete(&self, id: Uuid) -> Result<DeleteResult> {
        let _ = TabEntity::delete_by_id(id)
            .exec(self.db)
            .await
            .map_err(|e| anyhow::anyhow!(e));

        FolderEntity::delete_by_id(id)
            .exec(self.db)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    // --- Folder Queue Management ---

    /// Toggles a queue in a folder. If it exists, remove it. If not, add it.
    pub async fn toggle_queue_in_folder(&self, folder_id: Uuid, queue_id: Uuid) -> Result<bool> {
        let existing = FolderQueueEntity::find()
            .filter(FolderQueueColumn::FolderId.eq(folder_id))
            .filter(FolderQueueColumn::QueueId.eq(queue_id))
            .one(self.db)
            .await?;

        if let Some(entry) = existing {
            entry.delete(self.db).await?;
            Ok(false)
        } else {
            let new_entry = FolderQueueActiveModel {
                id: Set(Uuid::new_v4()),
                folder_id: Set(folder_id),
                queue_id: Set(queue_id),
                sort_order: Set(0), // Default order
                ..Default::default()
            };
            let _ = FolderQueueEntity::insert(new_entry)
                .exec_without_returning(self.db)
                .await?;
            Ok(true)
        }
    }

    /// Reorders queues within a folder using a transaction
    pub async fn reorder_queues(
        &self,
        folder_id: Uuid,
        ordered_queue_ids: Vec<Uuid>,
    ) -> Result<()> {
        self.db
            .transaction::<_, (), DbErr>(|txn| {
                Box::pin(async move {
                    for (index, q_id) in ordered_queue_ids.into_iter().enumerate() {
                        FolderQueueEntity::update_many()
                            .col_expr(FolderQueueColumn::SortOrder, Expr::value(index as i32))
                            .filter(FolderQueueColumn::FolderId.eq(folder_id))
                            .filter(FolderQueueColumn::QueueId.eq(q_id))
                            .exec(txn)
                            .await?;
                    }
                    Ok(())
                })
            })
            .await
            .map_err(|e| anyhow::anyhow!("Reorder transaction failed: {}", e))
    }

    /// Fetches all queues associated with a specific folder, ordered by sort_order
    pub async fn get_queues_for_folder(&self, folder_id: Uuid) -> Result<Vec<ReadQueueModel>> {
        let folder_queues = FolderQueueEntity::find()
            .filter(FolderQueueColumn::FolderId.eq(folder_id))
            .find_also_related(QueueEntity)
            .order_by_asc(FolderQueueColumn::SortOrder)
            .all(self.db)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        let result: Vec<ReadQueueModel> = folder_queues
            .into_iter()
            .filter_map(|(_, queue_opt)| queue_opt.map(|q| ReadQueueModel::from(q)))
            .collect();

        Ok(result)
    }
}
