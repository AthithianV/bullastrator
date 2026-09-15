use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // --- 1. Create the Table (Existing Code) ---
        manager
            .create_table(
                Table::create()
                    .table(Workspace::Table)
                    .if_not_exists()
                    // Primary Key
                    .col(
                        ColumnDef::new(Workspace::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Workspace::UserId).text())
                    // Workspace Details
                    .col(ColumnDef::new(Workspace::Name).text().not_null()) // Added UNIQUE to Name
                    .col(ColumnDef::new(Workspace::Color).text().null())
                    .col(ColumnDef::new(Workspace::ActiveTabId).text().null())
                    .col(ColumnDef::new(Workspace::Icon).integer().null())
                    .col(
                        ColumnDef::new(Workspace::LastAccessedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(Workspace::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(Workspace::Plan)
                            .string()
                            .not_null()
                            .default("FREE"),
                    ) // Stored as String for the Enum
                    .col(ColumnDef::new(Workspace::Role).string().not_null()) // Stored as String for the Enum
                    .col(
                        ColumnDef::new(Workspace::MaxConnections)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(Workspace::IsGuestMode)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        const DEFAULT_NAME: &str = "asyncian";

        let exists = manager
            .get_connection()
            .query_one(Statement::from_sql_and_values(
                manager.get_database_backend(),
                "SELECT 1 FROM workspace LIMIT 1;",
                vec![],
            ))
            .await?
            .is_some();

        if !exists {
            manager
                .get_connection()
                .execute(Statement::from_sql_and_values(
                    manager.get_database_backend(),
                    r#"INSERT INTO "workspace"
                           ("id", "name", "color", "plan", "role", "is_guest_mode")
                           VALUES (?,?,?,?,?,?)"#,
                    vec![
                        Uuid::new_v4().into(),
                        DEFAULT_NAME.into(),
                        "#00CADB".into(),
                        "FREE".into(),
                        "OWNER".into(),
                        true.into(),
                    ],
                ))
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Workspace::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Workspace {
    Table,
    Id,
    Name,
    Color,
    UserId,
    Icon,
    Plan,
    Role,
    MaxConnections,
    ActiveTabId,
    LastAccessedAt,
    CreatedAt,
    IsGuestMode,
}
