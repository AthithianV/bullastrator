use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Queue::Table)
                    .if_not_exists()
                    // Primary Key
                    .col(ColumnDef::new(Queue::Id).text().not_null().primary_key())
                    // Foreign Key to Connection
                    .col(ColumnDef::new(Queue::ConnectionId).text().not_null())
                    // Queue Identifier
                    .col(ColumnDef::new(Queue::QueueName).text().not_null())
                    // User Customization
                    .col(ColumnDef::new(Queue::DisplayName).text().null())
                    .col(ColumnDef::new(Queue::IsStarred).boolean().default(false))
                    .col(
                        ColumnDef::new(Queue::AutoRefreshRate)
                            .integer()
                            .default(5000),
                    )
                    .col(
                        ColumnDef::new(Queue::NotificationSettings)
                            .text()
                            .default("critical"),
                    )
                    // Timestamps
                    .col(
                        ColumnDef::new(Queue::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("idx-queue-connection-name")
                            .col(Queue::ConnectionId)
                            .col(Queue::QueueName),
                    )
                    // Foreign Key Constraint
                    .foreign_key(
                        ForeignKey::create()
                            .from(Queue::Table, Queue::ConnectionId)
                            .to(Connection::Table, Connection::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Queue::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Connection {
    Table,
    Id,
}

#[derive(Iden)]
enum Queue {
    Table,
    Id,
    ConnectionId,
    QueueName,
    DisplayName,
    AutoRefreshRate,
    IsStarred,
    NotificationSettings,
    CreatedAt,
}
