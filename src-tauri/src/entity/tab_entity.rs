use chrono::Utc;
use sea_orm::{ActiveValue::Set, entity::prelude::*};

use crate::model::tab_model::TabParams;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tab")] // or "tabs" depending on your preference
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    // Foreign Keys
    pub workspace_id: Uuid,
    pub connection_id: Option<Uuid>,

    // Content & Identity
    pub title: String,

    #[sea_orm(column_type = "Text")]
    pub params: TabParams,

    // UX State Booleans
    pub is_active: bool,
    pub is_dirty: bool,
    pub is_pinned: bool,
    pub is_preview: bool,

    // Ordering
    pub rank: i32,

    // Timestamps
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::connection_entity::Entity",
        from = "Column::ConnectionId",
        to = "super::connection_entity::Column::Id",
        on_delete = "Cascade"
    )]
    Connection,

    #[sea_orm(
        belongs_to = "super::workspace_entity::Entity",
        from = "Column::WorkspaceId",
        to = "super::workspace_entity::Column::Id",
        on_delete = "Cascade"
    )]
    Workspace,
}

// Reverse Relation: To access Connection from a Tab
impl Related<super::connection_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Connection.def()
    }
}

// Reverse Relation: To access Workspace from a Tab
impl Related<super::workspace_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workspace.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = Utc::now();

        if insert {
            self.created_at = Set(now);
        }

        // Always update 'updated_at' when saving
        self.updated_at = Set(now);

        Ok(self)
    }
}
