use crate::db;
use crate::domain::posts::Posts;
use crate::error_handler::CustomError;
use crate::schema::posts;
use diesel::prelude::*;

impl Posts {
    pub fn find_all() -> Result<Vec<Posts>, CustomError> {
        let mut conn = db::connection()?;
        let posts = posts::table.load::<Posts>(&mut conn)?;
        Ok(posts)
    }
}
