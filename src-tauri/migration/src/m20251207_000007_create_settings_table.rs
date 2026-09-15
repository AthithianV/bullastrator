use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Settings::Table)
                    .if_not_exists()
                    // Primary Key (typically used just to ensure a single row)
                    .col(
                        ColumnDef::new(Settings::Key)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Settings::Value).text())
                    .to_owned(),
            )
            .await?;

        const DEFAULT_WORKSPACE_NAME: &str = "asyncian";
        const WS_KEY: &str = "active_workspace_id";

        let exists = manager
            .get_connection()
            .query_one(Statement::from_sql_and_values(
                manager.get_database_backend(),
                "SELECT 1 FROM settings WHERE key = ?",
                vec![WS_KEY.into()],
            ))
            .await?
            .is_some();

        if !exists {
            manager
                .get_connection()
                .execute(Statement::from_sql_and_values(
                    manager.get_database_backend(),
                    r#"INSERT INTO "settings"
                           ("key", "value")
                           VALUES (?, ?)"#,
                    vec![WS_KEY.into(), DEFAULT_WORKSPACE_NAME.into()],
                ))
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Settings {
    Table,
    Key,
    Value,
}
