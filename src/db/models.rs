use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema::posts;

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: Option<bool>,
}
