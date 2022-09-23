use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2::Pool;
use std::env;

// The Postgres-specific connection pool managing all database connections.
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> PostgresPool {
    // it from the environment within this function
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("no DB URL");
    let migr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(migr)
        .expect("could not build connection pool")
}
