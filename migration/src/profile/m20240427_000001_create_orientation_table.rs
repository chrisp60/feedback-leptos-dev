use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager.create_table(Table::create().table(Orientations::Table)
                                            .if_not_exists()
                                            .col(ColumnDef::new(Orientations::Id).integer()
                                                                          .not_null()
                                                                          .auto_increment()
                                                                          .primary_key())
                                            .col(ColumnDef::new(Orientations::UserId).integer()
                                                                              .not_null())
                                            .col(ColumnDef::new(Orientations::Term).string()
                                                                               .not_null())
                                            .col(ColumnDef::new(Orientations::CreatedAt).date_time()
                                                                                    .not_null())
                                            .col(ColumnDef::new(Orientations::UpdatedAt).date_time()
                                                                                    .not_null())
                                            .to_owned())
               .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager.drop_table(Table::drop().table(Orientations::Table).to_owned())
               .await
    }
}

#[derive(DeriveIden)]
pub enum Orientations
{
    Table,
    Id,
    UserId,
    Term,
    CreatedAt,
    UpdatedAt
}
