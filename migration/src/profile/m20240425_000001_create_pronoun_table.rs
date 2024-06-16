use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration
{
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr>
	{
		manager.create_table(Table::create().table(Pronouns::Table)
		                                    .if_not_exists()
		                                    .col(ColumnDef::new(Pronouns::Id).integer().not_null().auto_increment().primary_key())
		                                    .col(ColumnDef::new(Pronouns::UserId).integer().not_null())
		                                    .col(ColumnDef::new(Pronouns::Term).string().not_null())
		                                    .col(ColumnDef::new(Pronouns::CreatedAt).date_time().not_null())
		                                    .col(ColumnDef::new(Pronouns::UpdatedAt).date_time().not_null())
		                                    .to_owned())
		       .await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr>
	{
		manager.drop_table(Table::drop().table(Pronouns::Table).to_owned()).await
	}
}

#[derive(DeriveIden)]
pub enum Pronouns
{
	Table,
	Id,
	UserId,
	Term,
	CreatedAt,
	UpdatedAt
}
