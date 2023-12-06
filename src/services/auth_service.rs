use bcrypt::{hash, DEFAULT_COST, verify};
use diesel::{PgConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::users;
use crate::utils::responses::{Error};

pub struct AuthService;

impl AuthService {
    pub fn hash_password(password: &str) -> String {
        hash(password, DEFAULT_COST).unwrap()
    }

    pub fn verify_password(password: &str, hashed_password: &str) -> bool {
        verify(password, hashed_password).unwrap()
    }

    pub fn register_user(mut conn: PooledConnection<ConnectionManager<PgConnection>>, register_data: NewUser) -> Result<(), Error> {
        use crate::schema::users::dsl;

        // Check if user with the same email already exists
        match dsl::users.filter(dsl::email.eq(&register_data.email)).first::<NewUser>(&mut conn) {
            Ok(_) => return Err(Error::new("Email already exists")),
            Err(diesel::NotFound) => (),
            Err(_) => return Err(Error::new("Database error")),
        };

        // Hash password and create new user instance
        let hashed_password = hash(&register_data.password, DEFAULT_COST)
            .map_err(|_| Error::new("Password hashing failed"))?;

        let new_user = NewUser {
            password: hashed_password,
            ..register_data
        };

        // Insert the new user into the database
        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut conn)
            .map_err(|_| Error::new("Failed to create user"))?;

        Ok(())
    }
}

