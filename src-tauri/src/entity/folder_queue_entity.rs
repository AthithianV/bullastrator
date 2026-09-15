use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use uuid::Uuid;

/// Entity generated from migration: `folder_queue`
///
/// Migration summary:
/// - id (integer, auto-increment, primary key)
/// - folder_id (integer, not null) → FK to `folder(id)` (on_delete: Cascade)
/// - queue_id (integer, null) → FK to `queue(id)` (on_delete: Cascade)
/// - sort_order (integer, null)
/// - created_at (timestamp, null, DEFAULT CURRENT_TIMESTAMP)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "folder_queue")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub folder_id: Uuid,
    pub queue_id: Uuid,
    pub sort_order: i32,

    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::folder_entity::Entity",
        from = "Column::FolderId",
        to = "super::folder_entity::Column::Id",
        on_delete = "Cascade"
    )]
    Folder,

    #[sea_orm(
        belongs_to = "super::queue_entity::Entity",
        from = "Column::QueueId",
        to = "super::queue_entity::Column::Id",
        on_delete = "Cascade"
    )]
    Queue,
}

impl Related<super::folder_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Folder.def()
    }
}

impl Related<super::queue_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Queue.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// Populate `created_at` on insert to mirror the migration's CURRENT_TIMESTAMP default.
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            if self.id.is_not_set() {
                self.id = Set(Uuid::new_v4());
            }
            self.created_at = Set(Utc::now());
        }
        Ok(self)
    }
}
