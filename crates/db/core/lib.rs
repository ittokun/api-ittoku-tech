mod mutation;
mod query;

pub use mutation::Mutation;
pub use query::Query;

use migration::Migrator;

use entity::{
    post,
    sea_orm::{Database, DbErr},
};
use migration::sea_orm_migration::{MigratorTrait, SchemaManager};

use std::env;

pub async fn run() -> Result<(), DbErr> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let db = Database::connect(db_url).await?;
    Migrator::up(&db, None).await?;

    let schema_manager = SchemaManager::new(&db);
    assert!(schema_manager.has_table("post").await?);

    let post = Mutation::create_post(
        &db,
        post::NewModel {
            title: "Hello".to_owned(),
            text: "World".to_owned(),
        },
    )
    .await
    .expect("Failed create post");
    let post = Query::find_post_by_id(&db, post.id.unwrap())
        .await
        .expect("Failed update post");
    assert_eq!(post.unwrap().title, "Hello");

    Mutation::delete_all_posts(&db).await.unwrap();

    Ok(())
}
