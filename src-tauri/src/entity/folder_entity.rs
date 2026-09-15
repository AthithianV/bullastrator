use chrono::Utc;
use sea_orm::{ActiveValue::Set, entity::prelude::*};
use uuid::Uuid;

/// Entity generated from migration: `folder`
///
/// Migration summary:
/// - id (integer, auto-increment, primary key)
/// - connection_id (integer, not null) → FK to `connection(id)` (on_delete: Cascade)
/// - title (string, not null)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "folder")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub connection_id: Uuid,
    pub title: String,

    pub created_at: Option<DateTimeUtc>,
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

    #[sea_orm(has_many = "super::folder_queue_entity::Entity")]
    FolderQueue,
}

impl Related<super::connection_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Connection.def()
    }
}

impl Related<super::folder_queue_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FolderQueue.def()
    }
}

impl Related<super::queue_entity::Entity> for Entity {
    fn to() -> RelationDef {
        super::folder_queue_entity::Relation::Queue.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::folder_queue_entity::Relation::Folder.def().rev())
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert && self.id.is_not_set() {
            self.id = Set(Uuid::new_v4());
            self.created_at = Set(Some(Utc::now()));
        }

        Ok(self)
    }
}
