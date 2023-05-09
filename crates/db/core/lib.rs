use entity::sea_orm::{Database, DatabaseConnection, DbErr};
use migration::sea_orm_migration::MigratorTrait;
use migration::Migrator;

use std::env;

pub use entity::sea_orm;
pub use mutation::Mutation;
pub use query::Query;

mod mutation;
mod query;

pub async fn get_db_connection() -> Result<DatabaseConnection, DbErr> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let db = Database::connect(db_url).await?;
    Ok(db)
}

pub async fn migrate() -> Result<(), DbErr> {
    let db = get_db_connection().await?;
    Migrator::up(&db, None).await?;
    Ok(())
}
