use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager.create_table(Table::create().table(Roles::Table)
                                            .if_not_exists()
                                            .col(ColumnDef::new(Roles::Id).integer()
                                                                          .not_null()
                                                                          .auto_increment()
                                                                          .primary_key())
                                            .col(ColumnDef::new(Roles::UserId).integer().not_null())
                                            .col(ColumnDef::new(Roles::Term).string().not_null())
                                            .col(ColumnDef::new(Roles::CreatedAt).date_time()
                                                                                 .not_null())
                                            .col(ColumnDef::new(Roles::UpdatedAt).date_time()
                                                                                 .not_null())
                                            .to_owned())
               .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
    {
        manager.drop_table(Table::drop().table(Roles::Table).to_owned())
               .await
    }
}

#[derive(DeriveIden)]
pub enum Roles
{
    Table,
    Id,
    UserId,
    Term,
    CreatedAt,
    UpdatedAt
}
