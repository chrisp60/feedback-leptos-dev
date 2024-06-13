use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager.create_table(Table::create().table(Anniversaries::Table)
                                            .if_not_exists()
                                            .col(ColumnDef::new(Anniversaries::Id).integer()
                                                                                .not_null()
                                                                                .auto_increment()
                                                                                .primary_key())
                                            .col(ColumnDef::new(Anniversaries::UserId).integer()
                                                                                    .not_null())
                                            .col(ColumnDef::new(Anniversaries::OtherId).integer()
                                                                                     .not_null())
                                            .col(ColumnDef::new(Anniversaries::Date).date()
                                                                                  .not_null())
                                            .col(ColumnDef::new(Anniversaries::CreatedAt).date_time()
                                                                                       .not_null())
                                            .col(ColumnDef::new(Anniversaries::UpdatedAt).date_time()
                                                                                       .not_null())
                                            .to_owned())
               .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager.drop_table(Table::drop().table(Anniversaries::Table).to_owned())
               .await
    }
}

#[derive(DeriveIden)]
pub enum Anniversaries
{
    Table,
    Id,
    UserId,
    OtherId,
    Date,
    CreatedAt,
    UpdatedAt
}
