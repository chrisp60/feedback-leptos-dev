use chrono::{NaiveDate, NaiveDateTime};
use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, FromJsonQueryResult)]
#[sea_orm(table_name = "users")]
pub struct Model
{
	#[sea_orm(primary_key)]
	#[serde(skip_deserializing)]
	pub id:            i32,
	#[sea_orm(unique)]
	pub username:      String,
	pub first_name:    String,
	pub last_name:     String,
	#[sea_orm(unique)]
	pub email:         String,
	pub password_hash: String,
	pub date_of_birth: NaiveDate,
	#[sea_orm(unique)]
	pub uuid:          Uuid,
	pub created_at:    NaiveDateTime,
	pub updated_at:    NaiveDateTime
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
	#[sea_orm(has_many = "super::post::Entity")]
	Post,
	#[sea_orm(has_many = "super::profile::anniversary::Entity")]
	Anniversary,
	#[sea_orm(has_one = "super::profile::biography::Entity")]
	Biography,
	#[sea_orm(has_many = "super::profile::gender::Entity")]
	Gender,
	#[sea_orm(has_one = "super::profile::location::Entity")]
	Location,
	#[sea_orm(has_many = "super::profile::orientation::Entity")]
	Orientation,
	#[sea_orm(has_many = "super::profile::pronoun::Entity")]
	Pronoun,
	#[sea_orm(has_many = "super::profile::relationship::Entity")]
	Relationship,
	#[sea_orm(has_many = "super::profile::role::Entity")]
	Role
}

impl Related<super::post::Entity> for Entity
{
	fn to() -> RelationDef
	{
		Relation::Post.def()
	}
}

impl Related<super::profile::anniversary::Entity> for Entity
{
	fn to() -> RelationDef
	{
		Relation::Anniversary.def()
	}
}

impl Related<super::profile::biography::Entity> for Entity
{
	fn to() -> RelationDef
	{
		Relation::Biography.def()
	}
}

impl Related<super::profile::gender::Entity> for Entity
{
	fn to() -> RelationDef
	{
		Relation::Gender.def()
	}
}

impl Related<super::profile::location::Entity> for Entity
{
	fn to() -> RelationDef
	{
		Relation::Location.def()
	}
}

impl Related<super::profile::orientation::Entity> for Entity
{
	fn to() -> RelationDef
	{
		Relation::Orientation.def()
	}
}

impl Related<super::profile::pronoun::Entity> for Entity
{
	fn to() -> RelationDef
	{
		Relation::Pronoun.def()
	}
}

impl Related<super::profile::relationship::Entity> for Entity
{
	fn to() -> RelationDef
	{
		Relation::Relationship.def()
	}
}

impl Related<super::profile::role::Entity> for Entity
{
	fn to() -> RelationDef
	{
		Relation::Role.def()
	}
}

impl ActiveModelBehavior for ActiveModel
{
}

pub struct RegisterUserModel
{
	pub username:      String,
	pub first_name:    String,
	pub last_name:     String,
	pub email:         String,
	pub date_of_birth: NaiveDate,
	pub password:      String
}
