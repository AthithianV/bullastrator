use chrono::{NaiveDateTime, Timelike, Utc};
pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20251207_000001_create_workspace_table;
mod m20251207_000002_create_connection_table;
mod m20251207_000003_create_tab_table;
mod m20251207_000004_create_folder_table;
mod m20251207_000005_create_queue_table;
mod m20251207_000006_create_folder_queue_table;
mod m20251207_000007_create_settings_table;
mod m20260402_000008_create_user_table;
mod m20260510_000009_add_is_primary_to_workspace;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251207_000002_create_connection_table::Migration),
            Box::new(m20251207_000001_create_workspace_table::Migration),
            Box::new(m20251207_000003_create_tab_table::Migration),
            Box::new(m20251207_000004_create_folder_table::Migration),
            Box::new(m20251207_000005_create_queue_table::Migration),
            Box::new(m20251207_000007_create_settings_table::Migration),
            Box::new(m20260402_000008_create_user_table::Migration),
            Box::new(m20251207_000006_create_folder_queue_table::Migration),
            Box::new(m20260510_000009_add_is_primary_to_workspace::Migration),
        ]
    }
}

pub fn now() -> NaiveDateTime {
    Utc::now().naive_local().with_nanosecond(0).unwrap()
}
