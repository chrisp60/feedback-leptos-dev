
use sea_orm::*;
use ls_entity::{post, post::Entity as Post };


pub struct PostMutation;

impl PostMutation 
{
    pub async fn create_post(db: &DbConn, form_data: post::Model) -> Result<post::ActiveModel, DbErr> 
    {
        post::ActiveModel 
        {
            user_id: Set(form_data.user_id.to_owned()),
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_post_by_id(db: &DbConn, id: i32, form_data: post::Model) -> Result<post::Model, DbErr> 
    {
        let post: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post::ActiveModel {
            id: post.id,
            user_id: Set(form_data.user_id.to_owned()),
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete_post(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let post: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    pub async fn delete_all_posts(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Post::delete_many().exec(db).await
    }
}
