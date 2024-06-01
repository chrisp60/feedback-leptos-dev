use chrono::NaiveDateTime;
use sea_orm::{entity::prelude::*, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone,
           Debug,
           PartialEq,
           Eq,
           DeriveEntityModel,
           Deserialize,
           Serialize,
           FromJsonQueryResult)]
#[sea_orm(table_name = "pronouns")]
pub struct Model
{
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id:         i32,
    pub user_id:    i32,
    pub term:       String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation
{
    #[sea_orm(belongs_to = "super::user::Entity",
              from = "Column::UserId",
              to = "super::user::Column::Id")]
    User
}

impl Related<super::user::Entity> for Entity
{
    fn to() -> RelationDef
    {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel
{
}
