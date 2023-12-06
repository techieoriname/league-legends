use dotenv::dotenv;
use std::env;
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct Env {
    pub environment: String,
    pub rust_log: String,
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_expires_in: i64,
}

impl Env {
    pub fn new() -> Self {
        dotenv().ok(); // Load .env file

        Self {
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost:5432/league_legends".to_string()),
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap_or(3001),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string()),
            jwt_expires_in: env::var("JWT_EXPIRES_IN").unwrap_or_else(|_| "24".to_string()).parse().unwrap_or(24),
        }
    }

    pub fn is_dev(&self) -> bool {
        self.environment == "development"
    }

    pub fn clone_env(&self) -> Self {
        self.clone()
    }
}

lazy_static! {
    pub static ref ENV: Env = Env::new();
}