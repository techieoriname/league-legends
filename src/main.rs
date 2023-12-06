use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::{io};
use crate::config::env::ENV;
use crate::config::db::establish_connection;

mod models;
mod config;
mod routes;
mod schema;
mod services;
mod controllers;
mod utils;

#[actix_web::main]
async fn main() -> io::Result<()> {
    if ENV.is_dev() {
        env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    }

    let env = ENV.clone_env();

    log::info!("Initializing MongoDB configuration...");

    let pool = establish_connection();

    let server_address = format!("127.0.0.1:{}", env.port);

    log::info!("Starting server at http://{}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .configure(routes::config)
    })
        .bind(server_address)?
        .run()
        .await
}
