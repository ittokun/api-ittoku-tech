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

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let post = posts::table.filter(posts::id.eq(id)).first(conn)?;
        Ok(post)
    }

    pub fn update(id: i32, post: Post) -> Result<Self, CustomError> {
        let conn = &mut db::connection()?;
        let post = diesel::update(posts::table)
            .filter(posts::id.eq(id))
            .set(post)
            .get_result(conn)?;
        Ok(post)
    }
}
