use crate::schema::posts;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub title: String,
    pub body: String,
    pub published: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = posts)]
pub struct Posts {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
