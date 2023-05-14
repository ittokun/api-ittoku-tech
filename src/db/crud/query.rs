use sea_orm::{DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryOrder};

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
}
