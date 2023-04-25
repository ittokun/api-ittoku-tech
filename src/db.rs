use crate::api_error::ApiError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use once_cell::sync::Lazy;
use std::env;

type Pool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;
type DbConnection = diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>>;

static POOL: Lazy<Pool> = Lazy::new(|| {
    let database_url = env::var("DATABASE_URL").expect("Database URL not set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create db pool")
});

pub fn init() {
    info!("Initializing DB");
}

pub fn connection() -> Result<DbConnection, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::InternalServerError(format!("Failed getting db connection: {}", e)))
}
