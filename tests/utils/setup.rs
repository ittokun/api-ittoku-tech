use api_ittoku_tech::db::database_connection;
use api_ittoku_tech::AppState;

use actix_web::web;

pub async fn app_state() -> web::Data<AppState>  {
    let conn = database_connection().await.unwrap();
    web::Data::new(AppState { conn })
}
