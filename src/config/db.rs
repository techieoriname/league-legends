use actix_web::{HttpResponse, web};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::pg::PgConnection;
use crate::config::env::ENV;

// Type alias for using the connection pool throughout your app
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    let database_url = ENV.clone_env().database_url;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn get_connection(pool: &web::Data<PgPool>) -> Result<PooledConnection<ConnectionManager<PgConnection>>, HttpResponse> {
    pool.get()
        .map_err(|e| {
            HttpResponse::InternalServerError().json("Internal server error")
        })
}