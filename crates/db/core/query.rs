use ::entity::{post, prelude::Post};

use ::entity::sea_orm::{DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryOrder};

pub struct Query;

impl Query {
    pub async fn find_post_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<post::Model>, DbErr> {
        Post::find_by_id(id).one(db).await
    }

    pub async fn find_posts_in_page(
        db: &DatabaseConnection,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<post::Model>, u64), DbErr> {
        let paginator = Post::find()
            .order_by_asc(post::Column::Id)
            .paginate(db, posts_per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
