use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use dotenvy;
use r2d2::Pool;
use std::env;

// The Postgres-specific connection pool managing all database connections.
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

const DB_URL_ERR: &str =
    "DATABASE_URL not found in environment.\nPlease use .env 'dot-env' or set variable on shell";

pub fn get_pool() -> DbPool {
    dotenvy::dotenv().expect("dotenvy error");
    let database_url = env::var("DATABASE_URL").expect(DB_URL_ERR);
    println!("Connecting to database on\n{} ......", database_url);

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("could not build connection pool")
}

pub fn establish_connection() -> DbPool {
    get_pool()
}
