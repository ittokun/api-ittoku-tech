use crate::db;
use crate::domain::posts::{Posts, Post};
use crate::error_handler::CustomError;
use crate::schema::posts;
use diesel::prelude::*;

impl Posts {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = &mut db::connection()?;
        let posts = posts::table.load::<Posts>(conn)?;
        Ok(posts)
    }

    pub fn create(post: Post) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let post = Post::from(post);
        let post = diesel::insert_into(posts::table)
            .values(post)
            .get_result(conn)?;
        Ok(post)
    }
}
