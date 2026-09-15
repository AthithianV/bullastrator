use anyhow::{Context, Result, bail};
use chrono::Utc;

use crate::entity::queue_entity::Column as QueueColumn;
use crate::entity::{
    ConnectionColumn, ConnectionEntity, ConnectionModel, QueueActiveModel, QueueEntity, QueueModel,
};
use crate::model::queue_model::{ConnectionWithQueues, ReadQueueModel, UpdateQueueModel};
use sea_orm::{
    ActiveModelTrait, DeleteResult, EntityTrait, Set, prelude::*, sea_query::OnConflict,
};

pub struct QueueRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> QueueRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn sync_all_queues(
        &self,
        connection_id: Uuid,
        queue_names: &Vec<String>,
        is_final_state: bool,
    ) -> Result<()> {
        let now = Utc::now();

        // ----------------------------
        // 1. Bulk Upsert (Optimized)
        // ----------------------------
        // We only attempt insert if there are names.
        // No need to fetch old names; Postgres handle conflicts.
        if !queue_names.is_empty() {
            let new_active_models: Vec<QueueActiveModel> = queue_names
                .iter()
                .map(|name| QueueActiveModel {
                    id: Set(Uuid::new_v4()),
                    connection_id: Set(connection_id),
                    queue_name: Set(name.clone()),
                    display_name: Set(Some(name.clone())),
                    is_starred: Set(Some(false)),
                    auto_refresh_rate: Set(Some(5000)),
                    notification_settings: Set(Some("critical".to_string())),
                    created_at: Set(now),
                })
                .collect();

            QueueEntity::insert_many(new_active_models)
                .on_conflict(
                    OnConflict::columns([QueueColumn::ConnectionId, QueueColumn::QueueName])
                        .do_nothing()
                        .to_owned(),
                )
                .exec_without_returning(self.db)
                .await?;
        }

        // ----------------------------
        // 2. Cleanup Logic (Corrected)
        // ----------------------------
        if is_final_state {
            let mut delete_query =
                QueueEntity::delete_many().filter(QueueColumn::ConnectionId.eq(connection_id));

            if !queue_names.is_empty() {
                // Case: Remove queues that exist in DB but were not in the Redis scan
                delete_query =
                    delete_query.filter(QueueColumn::QueueName.is_not_in(queue_names.clone()));
            }
            // Note: If queue_names IS empty, we don't add the is_not_in filter,
            // which results in deleting ALL queues for this connection_id.

            delete_query.exec(self.db).await?;
        }

        Ok(())
    }

    pub async fn get_all_by_workspace(
        &self,
        workspace_id: Uuid,
    ) -> Result<Vec<ConnectionWithQueues>> {
        // 1. Find all connections belonging to the workspace and include their queues
        // This uses the HasMany relationship defined in your Connection model
        let results: Vec<(ConnectionModel, Vec<QueueModel>)> = ConnectionEntity::find()
            .filter(ConnectionColumn::WorkspaceId.eq(workspace_id))
            .find_with_related(QueueEntity)
            .all(self.db)
            .await?;

        // 2. Map the results into your desired DTO format
        let grouped_results = results
            .into_iter()
            .map(|(connection, queues)| ConnectionWithQueues {
                id: connection.id,
                connection_name: connection.name,
                color: connection.color,
                queues: queues.into_iter().map(ReadQueueModel::from).collect(),
            })
            .collect();

        Ok(grouped_results)
    }

    /// Retrieves all queues associated with a connection ID.
    pub async fn get_all_by_connection(&self, connection_id: Uuid) -> Result<Vec<ReadQueueModel>> {
        let queue_models = QueueEntity::find()
            .filter(QueueColumn::ConnectionId.eq(connection_id))
            .all(self.db)
            .await
            .with_context(|| {
                format!("Failed to get all queues for connection {}", connection_id)
            })?;

        Ok(queue_models.into_iter().map(ReadQueueModel::from).collect())
    }

    // --- GET BY Name ---
    /// Retrieves a single queue by its name for a given connection ID.
    pub async fn get_by_name(
        &self,
        connection_id: i64,
        queue_name: &str,
    ) -> Result<Option<ReadQueueModel>> {
        let queue_model_opt = QueueEntity::find()
            .filter(QueueColumn::ConnectionId.eq(connection_id))
            .filter(QueueColumn::QueueName.eq(queue_name))
            .one(self.db)
            .await
            .with_context(|| {
                format!(
                    "Failed to get queue '{}' for connection {}",
                    queue_name, connection_id
                )
            })?;

        Ok(queue_model_opt.map(ReadQueueModel::from))
    }

    // --- UPDATE ---
    /// Updates an existing queue record by its primary key ID.
    pub async fn update(&self, id: Uuid, data: UpdateQueueModel) -> Result<ReadQueueModel> {
        // 1. Find existing record to convert to ActiveModel
        let model = QueueEntity::find_by_id(id)
            .one(self.db)
            .await
            .with_context(|| format!("Failed to find queue with ID: {} for update", id))?
            .ok_or_else(|| anyhow::anyhow!("Queue with ID {} not found", id))?;

        // 2. Convert to ActiveModel and apply updates
        let mut active: QueueActiveModel = model.into();

        if let Some(display_name) = data.display_name {
            active.display_name = Set(Some(display_name));
        }
        if let Some(is_starred) = data.is_starred {
            active.is_starred = Set(Some(is_starred));
        }
        if let Some(auto_refresh_rate) = data.auto_refresh_rate {
            active.auto_refresh_rate = Set(Some(auto_refresh_rate));
        }
        if let Some(notification_settings) = data.notification_settings {
            active.notification_settings = Set(Some(notification_settings));
        }

        // 3. Save updates
        let updated = active
            .update(self.db)
            .await
            .context("Failed to save updated queue to database")?;

        Ok(ReadQueueModel::from(updated))
    }

    // --- DELETE ---
    /// Deletes a queue record by its primary key ID.
    pub async fn delete(&self, id: Uuid) -> Result<DeleteResult> {
        let res = QueueEntity::delete_by_id(id)
            .exec(self.db)
            .await
            .with_context(|| format!("Failed to execute delete for queue ID {}", id))?;

        if res.rows_affected == 0 {
            bail!("Queue with ID {} not found", id);
        }

        Ok(res)
    }
}
