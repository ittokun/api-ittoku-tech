use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ModelTrait, PaginatorTrait, QueryOrder};

use crate::db::entities::{comment, prelude::Comment};
use crate::db::entities::{post, prelude::Post};

pub struct Query;

impl Query {
    pub async fn find_post_by_id(db: &DatabaseConnection, id: i32) -> Result<post::Model, DbErr> {
        let post = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(String::from("")))?;
        Ok(post)
    }

    pub async fn find_posts_in_page(
        db: &DatabaseConnection,
        page: u64,
        posts_per_page: u64,
    ) -> Result<post::ListModel, DbErr> {
        let paginator = Post::find()
            .order_by_asc(post::Column::Id)
            .paginate(db, posts_per_page);
        let total_count = paginator.num_pages().await?;

        paginator
            .fetch_page(page - 1)
            .await
            .map(|posts| post::ListModel { total_count, posts })
    }

    pub async fn find_comment_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<comment::Model, DbErr> {
        let comment = Comment::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(String::from("")))?;
        Ok(comment)
    }

    pub async fn find_post_comments_by_page(
        db: &DatabaseConnection,
        post_id: i32,
        page: u64,
        comments_per_page: u64,
    ) -> Result<comment::ListModel, DbErr> {
        let post = Self::find_post_by_id(db, post_id).await?;
        let paginator = post
            .find_related(Comment)
            .order_by_asc(comment::Column::Id)
            .paginate(db, comments_per_page);
        let total_count = paginator.num_pages().await?;

        paginator
            .fetch_page(page - 1)
            .await
            .map(|comments| comment::ListModel {
                total_count,
                comments,
            })
    }
}
