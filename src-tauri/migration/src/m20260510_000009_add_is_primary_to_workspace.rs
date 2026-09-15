use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Workspace::Table)
                    .add_column(
                        ColumnDef::new(Workspace::IsPrimary)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Workspace::Table)
                    .drop_column(Workspace::IsPrimary)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Workspace {
    Table,
    IsPrimary,
}
