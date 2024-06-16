use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
	{
		manager.create_table(Table::create().table(Locations::Table)
		                                    .if_not_exists()
		                                    .col(ColumnDef::new(Locations::Id).integer().not_null().auto_increment().primary_key())
		                                    .col(ColumnDef::new(Locations::UserId).integer().not_null())
		                                    .col(ColumnDef::new(Locations::Town).string().not_null())
		                                    .col(ColumnDef::new(Locations::County).string().not_null())
		                                    .col(ColumnDef::new(Locations::Country).string().not_null())
		                                    .col(ColumnDef::new(Locations::CreatedAt).date_time().not_null())
		                                    .col(ColumnDef::new(Locations::UpdatedAt).date_time().not_null())
		                                    .to_owned())
		       .await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
	{
		manager.drop_table(Table::drop().table(Locations::Table).to_owned()).await
	}
}

#[derive(DeriveIden)]
pub enum Locations
{
	Table,
	Id,
	UserId,
	Town,
	County,
	Country,
	CreatedAt,
	UpdatedAt
}
