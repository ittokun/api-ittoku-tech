use api_ittoku_tech::db::{database_connection, post, Mutation};
use api_ittoku_tech::AppState;

use actix_web::web;

pub async fn app_state() -> web::Data<AppState>  {
    let conn = database_connection().await.unwrap();
    web::Data::new(AppState { conn })
}

#[allow(dead_code)]
pub async fn create_post() -> post::Model {
    let conn = database_connection().await.unwrap();
    let post_data = post::NewModel {
        title: "test title".to_owned(),
        text: "test text".to_owned(),
    };
    Mutation::create_post(&conn, post_data).await.unwrap()
}

#[allow(dead_code)]
pub async fn delete_post(id: i32) -> post::Model {
    let conn = database_connection().await.unwrap();
    Mutation::delete_post_by_id(&conn, id).await.unwrap()
}
