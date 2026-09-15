use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FolderQueue::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FolderQueue::Id)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FolderQueue::FolderId).text().not_null())
                    .col(ColumnDef::new(FolderQueue::QueueId).text().not_null())
                    .col(
                        ColumnDef::new(FolderQueue::SortOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(FolderQueue::CreatedAt)
                            .timestamp()
                            .null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(FolderQueue::Table, FolderQueue::FolderId)
                            .to(Folder::Table, Folder::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(FolderQueue::Table, FolderQueue::QueueId)
                            .to(Queue::Table, Queue::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .col(FolderQueue::FolderId)
                            .col(FolderQueue::QueueId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FolderQueue::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum FolderQueue {
    Table,
    Id,
    FolderId,
    QueueId,
    SortOrder,
    CreatedAt,
}

#[derive(Iden)]
enum Folder {
    Table,
    Id,
}

#[derive(Iden)]
enum Queue {
    Table,
    Id,
}
