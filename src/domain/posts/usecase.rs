use crate::domain::posts::Posts;
use crate::error_handler::CustomError;

pub async fn find_all() -> Result<Vec<Posts>, CustomError> {
    let posts = Posts::find_all()?;
    Ok(posts)
}
