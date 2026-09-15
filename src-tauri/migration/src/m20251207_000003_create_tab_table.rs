use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tab::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tab::Id).string().not_null().primary_key())
                    // 2. Workspace is mandatory
                    .col(ColumnDef::new(Tab::WorkspaceId).text().not_null())
                    // 3. Connection is Optional (Nullable)
                    .col(ColumnDef::new(Tab::ConnectionId).text().null())
                    // 4. Content & Identity
                    .col(ColumnDef::new(Tab::Title).string().not_null())
                    .col(ColumnDef::new(Tab::Params).text().not_null()) // Stores "?queue=x&job=y"
                    // 5. VS Code UX State Booleans
                    .col(
                        ColumnDef::new(Tab::IsActive)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Tab::IsDirty)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Tab::IsPinned)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Tab::IsPreview)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    // 6. Ordering (Rank)
                    .col(ColumnDef::new(Tab::Rank).integer().not_null().default(0))
                    // 7. Timestamps
                    .col(
                        ColumnDef::new(Tab::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Tab::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    // Foreign Keys
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tab::Table, Tab::ConnectionId)
                            .to(Connection::Table, Connection::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tab::Table, Tab::WorkspaceId)
                            .to(Workspace::Table, Workspace::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tab::Table).to_owned())
            .await
    }
}

// Don't forget to update your Idens!
#[derive(Iden)]
enum Tab {
    Table,
    Id,
    WorkspaceId,
    ConnectionId,
    Title,
    Params,    // The new URL Query string field
    IsActive,  // Focus state
    IsDirty,   // Unsaved changes state
    IsPinned,  // Pinned state
    IsPreview, // Temporary tab state
    Rank,      // For drag-and-drop ordering
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Connection {
    Table,
    Id,
}

#[derive(Iden)]
enum Workspace {
    Table,
    Id,
}
