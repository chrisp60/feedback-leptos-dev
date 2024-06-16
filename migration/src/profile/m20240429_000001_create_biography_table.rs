use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
	{
		manager.create_table(Table::create().table(Biographies::Table)
		                                    .if_not_exists()
		                                    .col(ColumnDef::new(Biographies::Id).integer().not_null().auto_increment().primary_key())
		                                    .col(ColumnDef::new(Biographies::UserId).integer().not_null())
		                                    .col(ColumnDef::new(Biographies::Text).string().not_null())
		                                    .col(ColumnDef::new(Biographies::CreatedAt).date_time().not_null())
		                                    .col(ColumnDef::new(Biographies::UpdatedAt).date_time().not_null())
		                                    .to_owned())
		       .await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
	{
		manager.drop_table(Table::drop().table(Biographies::Table).to_owned()).await
	}
}

#[derive(DeriveIden)]
pub enum Biographies
{
	Table,
	Id,
	UserId,
	Text,
	CreatedAt,
	UpdatedAt
}
