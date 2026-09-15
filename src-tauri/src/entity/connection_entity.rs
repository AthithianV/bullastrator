use chrono::Utc;
use sea_orm::{ActiveValue::Set, entity::prelude::*};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "connection")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub password: Option<String>,
    pub username: Option<String>,
    pub db: i32,
    pub is_tls_enabled: bool,
    pub bullmq_prefix: String,

    pub color: Option<String>,
    pub label: Option<String>,

    pub last_synced_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tab_entity::Entity")]
    Tab,
    #[sea_orm(has_many = "super::folder_entity::Entity")]
    Folder,
    #[sea_orm(has_many = "super::queue_entity::Entity")]
    Queue,
    #[sea_orm(
        belongs_to = "super::workspace_entity::Entity",
        from = "Column::WorkspaceId",
        to = "super::workspace_entity::Column::Id",
        on_delete = "Cascade"
    )]
    Workspace,
}

impl Related<super::tab_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tab.def()
    }
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
        if insert {
            if self.id.is_not_set() {
                self.id = Set(Uuid::new_v4());
            }
            let now = Utc::now();
            self.created_at = Set(now);
        }

        Ok(self)
    }
}
