use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Folder::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Folder::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(Folder::ConnectionId).text().not_null())
                    .col(ColumnDef::new(Folder::Title).string().not_null())
                    .col(
                        ColumnDef::new(Folder::CreatedAt)
                            .timestamp()
                            .null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Folder::Table, Folder::ConnectionId)
                            .to(Connection::Table, Connection::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx-folder-title-connection-id")
                            .col(Folder::ConnectionId)
                            .col(Folder::Title),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Folder::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Folder {
    Table,
    Id,
    ConnectionId,
    Title,
    CreatedAt,
}

#[derive(Iden)]
enum Connection {
    Table,
    Id,
}
