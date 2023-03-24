use crate::api_error::ApiError;
use crate::db;
use crate::schema::posts;
use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, AsChangeset)]
#[diesel(table_name = posts)]
pub struct PostParams {
    pub title: String,
    pub body: String,
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

    pub fn create(post: PostParams) -> Result<Self, ApiError> {
        let conn = &mut db::connection()?;
        let post = Post::from(post);
        let post = diesel::insert_into(posts::table)
            .values(post)
            .get_result(conn)?;
        Ok(post)
    }
}

impl From<PostParams> for Post {
    fn from(value: PostParams) -> Self {
        Post {
            id: Uuid::new_v4(),
            title: value.title,
            body: value.body,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
