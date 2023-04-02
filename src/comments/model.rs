use crate::api_error::ApiError;
use crate::db;
use crate::posts::Post;
use crate::schema::comments;
use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations, Insertable)]
#[diesel(belongs_to(Post))]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: Uuid,
    pub body: String,
    pub post_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Validate, AsChangeset)]
#[diesel(table_name = comments)]
pub struct CommentParams {
    #[validate(custom = "validate_body")]
    pub body: String,
    pub updated_at: Option<NaiveDateTime>,
}

impl Comment {
    pub async fn find_all(post_id: Uuid) -> Result<Vec<Self>, ApiError> {
        let conn = &mut db::connection()?;
        let post = Post::find(post_id)?;
        let comments = Comment::belonging_to(&post)
            .select(Comment::as_select())
            .load(conn)?;
        Ok(comments)
    }

    pub async fn create(post_id: Uuid, comment: CommentParams) -> Result<Self, ApiError> {
        let conn = &mut db::connection()?;
        let comment = Comment::from_belong_post(post_id, comment);
        let comment = diesel::insert_into(comments::table)
            .values(comment)
            .get_result(conn)?;
        Ok(comment)
    }

    fn from_belong_post(post_id: Uuid, comment: CommentParams) -> Self {
        Comment {
            id: Uuid::new_v4(),
            body: comment.body,
            post_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CommentFindAll {
    pub total_count: usize,
    pub comments: Vec<Comment>,
}

impl CommentFindAll {
    pub fn new(total_count: usize, comments: Vec<Comment>) -> Self {
        CommentFindAll {
            total_count,
            comments,
        }
    }
}

fn validate_body(body: &str) -> Result<(), ValidationError> {
    if body.is_empty() {
        return Err(ValidationError::new("body is required"));
    } else if body.len() > 65536 {
        return Err(ValidationError::new("body is too long"));
    }

    Ok(())
}
