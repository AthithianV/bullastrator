pub mod connection_entity;
pub mod folder_entity;
pub mod folder_queue_entity;
pub mod queue_entity;
pub mod settings_entity;
pub mod tab_entity;
pub mod user_entity;
pub mod workspace_entity;

pub use connection_entity::{
    ActiveModel as ConnectionActiveModel, Column as ConnectionColumn, Entity as ConnectionEntity,
    Model as ConnectionModel,
};

pub use settings_entity::{
    ActiveModel as SettingsActiveModel, Column as SettingsColumn, Entity as SettingsEntity,
    Model as SettingsModel,
};

pub use workspace_entity::{
    ActiveModel as WorkspaceActiveModel, Column as WorkspaceColumn, Entity as WorkspaceEntity,
    Model as WorkspaceModel,
};

pub use queue_entity::{
    ActiveModel as QueueActiveModel, Column as QueueColumn, Entity as QueueEntity,
    Model as QueueModel,
};

pub use tab_entity::{
    ActiveModel as TabActiveModel, Column as TabColumn, Entity as TabEntity, Model as TabModel,
};

pub use folder_entity::{
    ActiveModel as FolderActiveModel, Column as FolderColumn, Entity as FolderEntity,
    Model as FolderModel,
};

pub use folder_queue_entity::{
    ActiveModel as FolderQueueActiveModel, Column as FolderQueueColumn,
    Entity as FolderQueueEntity, Model as FolderQueueModel,
};

pub use user_entity::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity, Model as UserModel,
};
