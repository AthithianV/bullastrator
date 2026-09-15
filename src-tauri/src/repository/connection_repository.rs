use uuid::Uuid;

use crate::entity::{ConnectionActiveModel, ConnectionColumn, ConnectionEntity};
use crate::model::connection_model::{
    CreateConnectionModel, LessSecureConnectionModel, ReadConnectionModel, UpdateConnectionModel,
};
use crate::utilities::encryption::encrypt_password;
use anyhow::{Context, Result, bail};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DeleteResult, EntityTrait, QueryFilter, Set, prelude::*,
};

pub struct ConnectionRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> ConnectionRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    // --- CREATE ---
    pub async fn create(
        &self,
        data: CreateConnectionModel,
        workspace_id: Uuid,
    ) -> Result<ReadConnectionModel> {
        let existing_connection_name = ConnectionEntity::find()
            .filter(ConnectionColumn::Name.eq(data.name.clone()))
            .filter(ConnectionColumn::WorkspaceId.eq(workspace_id))
            .one(self.db)
            .await
            .with_context(|| "Failed to fetch connections from database")?;

        if existing_connection_name.is_some() {
            bail!("Connection with name {} already exists", data.name);
        }

        let encrypted_password = match data.password {
            Some(password) => {
                let encrypted_password = encrypt_password(&password)
                    .map_err(|e| anyhow::anyhow!("Failed to encrypt password: {}", e))?;
                Some(encrypted_password)
            }
            None => None,
        };

        let bullmq_prefix = data.bullmq_prefix.unwrap_or("bull".to_string());

        let id = Uuid::new_v4();
        let new_connection = ConnectionActiveModel {
            id: Set(id),
            workspace_id: Set(workspace_id),
            name: Set(data.name),
            host: Set(data.host),
            port: Set(data.port),
            password: Set(encrypted_password),
            username: Set(data.username),
            db: Set(data.db.unwrap_or(0)),
            is_tls_enabled: Set(data.is_tls_enabled),
            bullmq_prefix: Set(bullmq_prefix),
            created_at: Default::default(),
            last_synced_at: Set(None),
            color: Set(data.color),
            label: Set(data.label),
        };

        let _ = ConnectionEntity::insert(new_connection)
            .exec_without_returning(self.db)
            .await
            .with_context(|| "Failed to create new Connection")?;

        let inserted_row_option = ConnectionEntity::find_by_id(id).one(self.db).await?;

        let inserted_row = match inserted_row_option {
            Some(row) => row,
            None => bail!("Could not insert a row"),
        };

        let result = ReadConnectionModel::from(inserted_row);
        Ok(result)
    }

    // --- GET ALL ---
    pub async fn get_all(&self, workspace_id: Uuid) -> Result<Vec<ReadConnectionModel>> {
        let connection_model_vec = ConnectionEntity::find()
            .filter(ConnectionColumn::WorkspaceId.eq(workspace_id))
            .all(self.db)
            .await
            .with_context(|| "Failed to fetch connections from database")?;

        Ok(connection_model_vec
            .into_iter()
            .map(ReadConnectionModel::from)
            .collect())
    }

    pub async fn count_by_workspace(&self, workspace_id: Uuid) -> Result<u32> {
        let count = ConnectionEntity::find()
            .filter(ConnectionColumn::WorkspaceId.eq(workspace_id))
            .count(self.db)
            .await
            .with_context(|| {
                format!(
                    "Failed to count connections for workspace: {}",
                    workspace_id
                )
            })?;

        Ok(count as u32)
    }

    // --- GET BY ID ---
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<ReadConnectionModel>> {
        let connection_model_opt = ConnectionEntity::find_by_id(id)
            .one(self.db)
            .await
            .with_context(|| format!("Failed to get connection for {}", id))?;

        Ok(connection_model_opt.map(ReadConnectionModel::from))
    }

    pub async fn get_by_id_with_password(
        &self,
        id: Uuid,
    ) -> Result<Option<LessSecureConnectionModel>> {
        let connection_model_opt = ConnectionEntity::find_by_id(id)
            .one(self.db)
            .await
            .with_context(|| format!("Failed to get connection for {}", id))?;

        Ok(connection_model_opt.map(LessSecureConnectionModel::from))
    }

    // --- UPDATE ---
    pub async fn update(
        &self,
        id: Uuid,
        data: UpdateConnectionModel,
    ) -> Result<ReadConnectionModel> {
        // 1. Find existing record (Must be converted to ActiveModel to update)
        let conn_model = ConnectionEntity::find_by_id(id)
            .one(self.db)
            .await
            .with_context(|| format!("Failed to find connection with ID: {} for Update", id))?
            .ok_or_else(|| anyhow::anyhow!("Connection with ID {} not found", id))?; // Use anyhow to bail

        // 2. Convert to ActiveModel
        let mut active: ConnectionActiveModel = conn_model.into();

        // 3. Update only the fields that are Some(...)
        if let Some(name) = data.name {
            active.name = Set(name);
        }
        if let Some(host) = data.host {
            active.host = Set(host);
        }
        if let Some(port) = data.port {
            active.port = Set(port);
        }
        if let Some(username) = data.username {
            active.username = Set(Some(username));
        }

        if let Some(label) = data.label {
            active.label = Set(Some(label));
        }

        if let Some(color) = data.color {
            active.color = Set(Some(color));
        }
        if let Some(password) = data.password {
            let encrypted_password = encrypt_password(&password)
                .map_err(|e| anyhow::anyhow!("Failed to encrypt password: {}", e))?;
            active.password = Set(Some(encrypted_password));
        }
        if let Some(db) = data.db {
            active.db = Set(db);
        }
        if let Some(bullmq_prefix) = data.bullmq_prefix {
            active.bullmq_prefix = Set(bullmq_prefix);
        }

        if let Some(is_tls_enabled) = data.is_tls_enabled {
            active.is_tls_enabled = Set(is_tls_enabled);
        }

        // 4. Save updates. ActiveModel only updates Set fields.
        let updated = active
            .update(self.db)
            .await
            .context("Failed to save updated connection to database")?;

        Ok(ReadConnectionModel::from(updated))
    }

    // --- DELETE ---
    pub async fn delete(&self, id: Uuid) -> Result<DeleteResult> {
        let res = ConnectionEntity::delete_by_id(id)
            .exec(self.db)
            .await
            .with_context(|| format!("Failed to execute delete for connection ID {}", id))?;

        if res.rows_affected == 0 {
            bail!("Connection with ID {} not found", id);
        }

        Ok(res)
    }
}
