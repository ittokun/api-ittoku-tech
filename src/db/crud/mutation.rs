use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};

use crate::db::crud::query::Query;
use crate::db::entities::{post, prelude::Post};

pub struct Mutation;

impl Mutation {
    pub async fn create_post(
        db: &DatabaseConnection,
        form_data: post::NewModel,
    ) -> Result<post::Model, DbErr> {
        post::ActiveModel {
            title: Set(form_data.title),
            text: Set(form_data.text),
            ..Default::default()
        }
        .insert(db)
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

    pub async fn delete_post_by_id(db: &DatabaseConnection, id: i32) -> Result<post::Model, DbErr> {
        let post = Query::find_post_by_id(db, id).await?;
        Post::delete_by_id(post.id).exec(db).await?;

        Ok(post)
    }

    // pub async fn delete_all_posts(db: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
    //     Post::delete_many().exec(db).await
    // }
}
