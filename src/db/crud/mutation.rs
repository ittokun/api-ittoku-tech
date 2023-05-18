use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

use crate::db::crud::query::Query;
use crate::db::entities::{comment, prelude::Comment};
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
            .ok_or(DbErr::RecordNotFound("Cannot Find Post".to_owned()))
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
        Comment::delete_many()
            .filter(comment::Column::PostId.eq(id))
            .exec(db)
            .await?;
        Post::delete_by_id(post.id).exec(db).await?;

        Ok(post)
    }

    // pub async fn delete_all_posts(db: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
    //     Post::delete_many().exec(db).await
    // }

    pub async fn create_comment(
        db: &DatabaseConnection,
        form_data: comment::NewModel,
    ) -> Result<comment::Model, DbErr> {
        comment::ActiveModel {
            text: Set(form_data.text),
            post_id: Set(form_data.post_id),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn update_comment_by_id(
        db: &DatabaseConnection,
        id: i32,
        form_data: comment::NewModel,
    ) -> Result<comment::Model, DbErr> {
        let comment: comment::ActiveModel = Comment::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot Find Comment".to_owned()))
            .map(Into::into)?;

        comment::ActiveModel {
            id: comment.id,
            text: Set(form_data.text),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_comment_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<comment::Model, DbErr> {
        let comment = Query::find_comment_by_id(db, id).await?;
        Comment::delete_by_id(comment.id).exec(db).await?;

        Ok(comment)
    }
}
