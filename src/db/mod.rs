pub mod users;
pub mod sessions;

use thiserror::Error;
use diesel::result;
use diesel::pg::PgConnection;
use diesel::r2d2::{self,ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;


pub type DBPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Error)]
pub enum DBError {
    #[error("Database connection pool error.")]
    PoolErr(#[from] r2d2::PoolError),
    #[error("Database error: {0}")]
    DatabaseErr(#[from] result::Error),
    #[error("Session expired")]
    SessionExpired,
    #[error("Internal error {0}")]
    InternalErr(String),
}
pub type DBResult<T> = Result<T,DBError>;

// Initialize a database connection pool during application startup
pub fn create_pool() -> DBPool {
    dotenv().ok();
    let host = env::var("DB_HOST").expect("DB_HOST must be set");
    let port = env::var("DB_PORT").expect("DB_PORT must be set");
    let dbname = env::var("DB_DBNAME").expect("DB_DBNAME must be set");
    let username = env::var("DB_USERNAME").expect("DB_NAME must be set");
    let password = env::var("DB_PASS").expect("DB_PASS must be set");

    let full_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        username, password, host, port, dbname
    );    
    let manager = ConnectionManager::<PgConnection>::new(full_url);
    Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool.")
}