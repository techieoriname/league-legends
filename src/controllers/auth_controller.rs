use actix_web::{HttpResponse, web};
use actix_web::http::StatusCode;
use serde_json::{from_slice, json};
use validator::Validate;
use crate::config::db::{get_connection, PgPool};
use crate::models::user::NewUser;
use crate::services::auth_service::AuthService;
use crate::utils::helpers::format_validation_errors;
use crate::utils::responses::{Error, error_response, success_response};

pub fn register(dbc: web::Data<PgPool>, body: web::Bytes) -> HttpResponse {
    let db = match get_connection(&dbc) {
        Ok(conn) => conn,
        Err(response) => return response
    };

    let register_data_result: Result<NewUser, serde_json::Error> =
        from_slice(&body);

    match register_data_result {
        Ok(register_data) => {
            if let Err(errors) = register_data.validate() {
                let error_message = format_validation_errors(errors);
                return error_response(
                    &Error::new("Validation error"),
                    StatusCode::BAD_REQUEST,
                )
            }

            match AuthService::register_user(db, register_data) {
                Ok(_) => success_response(StatusCode::OK, "User registered successfully", None),
                Err(e) => error_response(&e, StatusCode::BAD_REQUEST),
            }
        }
        Err(_) => {
            return error_response(
                &Error::new("Validation error"),
                StatusCode::BAD_REQUEST
            );
        }

    }


    HttpResponse::Ok().json(json!({"message": "Hello, world!"}))
}

pub fn login() -> HttpResponse {
    HttpResponse::Ok().json(json!({"message": "Hello, world!"}))
}