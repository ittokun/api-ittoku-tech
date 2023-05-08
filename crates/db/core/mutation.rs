use ::entity::sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, Set,
};
use ::entity::{post, prelude::*};

use chrono::Utc;

pub struct Mutation;

impl Mutation {
    pub async fn create_post(
        db: &DatabaseConnection,
        form_data: post::NewModel,
    ) -> Result<post::ActiveModel, DbErr> {
        post::ActiveModel {
            title: Set(form_data.title),
            text: Set(form_data.text),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_post_by_id(
        db: &DatabaseConnection,
        id: i32,
        form_data: post::NewModel,
    ) -> Result<post::Model, DbErr> {
        let post: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot Find Post".to_owned()))
            .map(Into::into)?;

        post::ActiveModel {
            id: post.id,
            title: Set(form_data.title),
            text: Set(form_data.text),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_post_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<DeleteResult, DbErr> {
        let post: post::ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot Find Post".to_owned()))
            .map(Into::into)?;

        post.delete(db).await
    }

    pub async fn delete_all_posts(db: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
        Post::delete_many().exec(db).await
    }
}
