use crate::{
    entity::{ConnectionModel, tab_entity},
    model::{
        connection_model::ReadConnectionModel, folder_model::FolderTabParams,
        queue_model::QueueTabParams,
    },
};
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadTabModel {
    pub id: String,
    pub workspace_id: Uuid,
    pub connection_id: Option<Uuid>,

    pub title: String,
    pub params: TabParams,

    // UX State
    pub is_active: bool,
    pub is_dirty: bool,
    pub is_pinned: bool,
    pub is_preview: bool,
    pub connection: Option<ReadConnectionModel>,

    pub rank: i32,
}

impl ReadTabModel {
    pub fn from_with_connection(entity: tab_entity::Model, conn: Option<ConnectionModel>) -> Self {
        ReadTabModel {
            id: entity.id,
            workspace_id: entity.workspace_id,
            connection_id: entity.connection_id,
            title: entity.title,
            params: entity.params,
            is_active: entity.is_active,
            is_dirty: entity.is_dirty,
            is_pinned: entity.is_pinned,
            is_preview: entity.is_preview,
            rank: entity.rank,
            connection: conn.map(ReadConnectionModel::from),
        }
    }
}

impl From<tab_entity::Model> for ReadTabModel {
    fn from(entity: tab_entity::Model) -> Self {
        ReadTabModel {
            id: entity.id,
            workspace_id: entity.workspace_id,
            connection_id: entity.connection_id,
            title: entity.title,
            params: entity.params,
            is_active: entity.is_active,
            is_dirty: entity.is_dirty,
            is_pinned: entity.is_pinned,
            is_preview: entity.is_preview,
            rank: entity.rank,
            connection: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, FromJsonQueryResult)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum TabParams {
    #[serde(rename = "QUEUE")]
    QueueTabParams(QueueTabParams),
    #[serde(rename = "FOLDER")]
    FolderTabParams(FolderTabParams),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTabModel {
    pub connection_id: Option<Uuid>,
    pub title: String,
    pub params: TabParams,
    pub is_preview: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTabModel {
    pub title: Option<String>,
    pub params: Option<TabParams>,
    pub rank: Option<i32>,

    pub is_active: Option<bool>,
    pub is_dirty: Option<bool>,
    pub is_pinned: Option<bool>,
    pub is_preview: Option<bool>,
}
