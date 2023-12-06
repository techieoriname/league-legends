use actix_web::{web, HttpResponse};
use serde_json::json;
use crate::controllers::auth_controller;

async fn index() -> HttpResponse {
    HttpResponse::Ok().json(json!({"message": "Hello, world!"}))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/auth/register").route(web::post().to(auth_controller::register)))
        .service(web::resource("/auth/login").route(web::post().to(auth_controller::login)));
}
