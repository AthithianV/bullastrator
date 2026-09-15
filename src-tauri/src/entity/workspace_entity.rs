use chrono::Utc;
use sea_orm::{ActiveValue::Set, ConnectionTrait, entity::prelude::*};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "workspace")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Option<String>,
    pub name: String,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub active_tab_id: Option<String>,
    pub last_accessed_at: DateTimeUtc,
    pub plan: String,
    pub role: String,
    pub max_connections: u32,
    pub is_guest_mode: bool,
    pub is_primary: bool,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::connection_entity::Entity")]
    Connection,
}

impl Related<super::connection_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Connection.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.id = Set(Uuid::new_v4());
            let now = Utc::now();
            self.created_at = Set(now);
        }

        Ok(self)
    }
}
