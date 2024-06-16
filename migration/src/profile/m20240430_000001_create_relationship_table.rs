use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
	{
		manager.create_table(Table::create().table(Relationships::Table)
		                                    .if_not_exists()
		                                    .col(ColumnDef::new(Relationships::Id).integer().not_null().auto_increment().primary_key())
		                                    .col(ColumnDef::new(Relationships::UserId).integer().not_null())
		                                    .col(ColumnDef::new(Relationships::OtherId).integer().not_null())
		                                    .col(ColumnDef::new(Relationships::Term).string().not_null())
		                                    .col(ColumnDef::new(Relationships::CreatedAt).date_time().not_null())
		                                    .col(ColumnDef::new(Relationships::UpdatedAt).date_time().not_null())
		                                    .to_owned())
		       .await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
	{
		manager.drop_table(Table::drop().table(Relationships::Table).to_owned()).await
	}
}

#[derive(DeriveIden)]
pub enum Relationships
{
	Table,
	Id,
	UserId,
	OtherId,
	Term,
	CreatedAt,
	UpdatedAt
}
