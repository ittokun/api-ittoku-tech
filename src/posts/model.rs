use crate::api_error::ApiError;
use crate::db;
use crate::schema::posts;
use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Queryable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Post {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = &mut db::connection()?;
        let posts = posts::table.load::<Post>(conn)?;
        Ok(posts)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = &mut db::connection()?;
        let post = posts::table.filter(posts::id.eq(id)).first(conn)?;
        Ok(post)
    }
}
