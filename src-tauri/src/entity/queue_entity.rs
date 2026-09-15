use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use uuid::Uuid;

/// Entity generated from migration: `queue`
///
/// Migration summary:
/// - id (integer, auto-increment, primary key)
/// - connection_id (integer, not null) → FK to `connection(id)` (on_delete: Cascade)
/// - queue_name (text, not null)
/// - display_name (text, null)
/// - is_starred (boolean, default false)
/// - auto_refresh_rate (integer, default 5000)
/// - notification_settings (text, default "critical")
/// - created_at (timestamp, not null, DEFAULT CURRENT_TIMESTAMP)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "queue")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    pub connection_id: Uuid,
    pub queue_name: String,
    pub display_name: Option<String>,

    /// Whether the user starred this queue
    pub is_starred: Option<bool>,

    /// Auto refresh rate in milliseconds
    pub auto_refresh_rate: Option<i32>,

    /// e.g. "critical", "all", etc.
    pub notification_settings: Option<String>,

    pub created_at: DateTimeUtc,
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

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
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
