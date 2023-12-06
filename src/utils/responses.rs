use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde_json::{json, Value};

pub fn success_response(status: StatusCode, message: &str, data: Option<Value>) -> HttpResponse {
    let mut response_body = json!({
        "status": "success",
        "message": message,
    });

    if let Some(data) = data {
        response_body["data"] = data;
    }

    HttpResponse::build(status).json(response_body)
}

pub fn error_response(error: &Error, status: StatusCode) -> HttpResponse {
    let error_message = json!({
        "status": "error",
        "error": error.message()
    });

    HttpResponse::build(status).json(error_message)
}

pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: &str) -> Error {
        Error { message: message.to_string() }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}