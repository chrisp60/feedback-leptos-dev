use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager.create_table(Table::create().table(Genders::Table)
                                            .if_not_exists()
                                            .col(ColumnDef::new(Genders::Id).integer()
                                                                            .not_null()
                                                                            .auto_increment()
                                                                            .primary_key())
                                            .col(ColumnDef::new(Genders::UserId).integer()
                                                                                .not_null())
                                            .col(ColumnDef::new(Genders::Term).string().not_null())
                                            .col(ColumnDef::new(Genders::CreatedAt).date_time()
                                                                                   .not_null())
                                            .col(ColumnDef::new(Genders::UpdatedAt).date_time()
                                                                                   .not_null())
                                            .to_owned())
               .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager.drop_table(Table::drop().table(Genders::Table).to_owned())
               .await
    }
}

#[derive(DeriveIden)]
pub enum Genders
{
    Table,
    Id,
    UserId,
    Term,
    CreatedAt,
    UpdatedAt
}
