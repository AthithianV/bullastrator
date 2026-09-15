use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Connection::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Connection::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Connection::WorkspaceId).text().not_null())
                    .col(
                        ColumnDef::new(Connection::Name)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Connection::Host).text().not_null())
                    .col(ColumnDef::new(Connection::Port).integer().not_null())
                    .col(ColumnDef::new(Connection::Password).text().null())
                    .col(ColumnDef::new(Connection::Username).text().null())
                    .col(ColumnDef::new(Connection::Db).integer().default(0))
                    .col(ColumnDef::new(Connection::LastSyncedAt).integer().null())
                    .col(
                        ColumnDef::new(Connection::BullmqPrefix)
                            .text()
                            .default("bull"),
                    )
                    .col(
                        ColumnDef::new(Connection::IsTlsEnabled)
                            .boolean()
                            .default(false),
                    )
                    .col(ColumnDef::new(Connection::Color).text().null())
                    .col(ColumnDef::new(Connection::Label).text().null())
                    .col(
                        ColumnDef::new(Connection::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx-connection-workspace-name")
                            .col(Connection::WorkspaceId)
                            .col(Connection::Name),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Connection::Table, Connection::WorkspaceId)
                            .to(Workspace::Table, Workspace::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Connection::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Connection {
    Table,
    Id,
    WorkspaceId,
    Name,
    Host,
    Port,
    Username,
    Password,
    Db,
    Color,
    Label,
    LastSyncedAt,
    BullmqPrefix,
    IsTlsEnabled,
    CreatedAt,
}

#[derive(Iden)]
enum Workspace {
    Table,
    Id,
}
