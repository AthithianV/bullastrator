use crate::entity::{UserActiveModel, UserEntity, user_entity};
use crate::model::user_model::ReadUserModel;
use anyhow::{Context, Result};
use sea_orm::*;

pub struct UserRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> UserRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn upsert(&self, data: &ReadUserModel) -> Result<ReadUserModel> {
        let active_model = UserActiveModel {
            id: Set(data.id.clone()),
            name: Set(data.name.clone()),
            email: Set(data.email.clone()),
            image: Set(data.image.clone()),
            plan: Set(data.plan.clone()),
            max_members: Set(data.max_members),
            max_workspaces: Set(data.max_workspaces),
            // Ensure these are set for the initial insert
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
        };

        // 1. Define the conflict logic
        let on_conflict = sea_query::OnConflict::column(user_entity::Column::Id)
            .update_columns([
                user_entity::Column::Name,
                user_entity::Column::Email,
                user_entity::Column::Image,
                user_entity::Column::Plan,
                user_entity::Column::MaxMembers,
                user_entity::Column::MaxWorkspaces,
                user_entity::Column::UpdatedAt,
            ])
            .to_owned();

        // 2. Execute with explicit model mapping
        let result = UserEntity::insert(active_model)
            .on_conflict(on_conflict)
            .exec_with_returning(self.db)
            .await
            .context("Failed to upsert user")?;

        Ok(ReadUserModel::from(result))
    }

    pub async fn get_current_user(&self) -> Result<Option<ReadUserModel>> {
        let user = UserEntity::find().one(self.db).await?;

        Ok(user.map(ReadUserModel::from))
    }

    pub async fn delete_current_user(&self) -> Result<()> {
        let _ = UserEntity::delete_many().exec(self.db).await?;
        Ok(())
    }
}
