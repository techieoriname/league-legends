use crate::schema::users;
use diesel::{Insertable, Queryable};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, NaiveDate};
use validator_derive::Validate;

#[derive(Queryable, Serialize, Deserialize, Validate)]
pub struct User {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

#[derive(Queryable, Insertable, Deserialize, Serialize, Validate)]
#[table_name="users"]
pub struct NewUser {
    #[validate(length(min = 2, message = "First name must be at least 2 characters long"))]
    pub first_name: String,

    pub middle_name: Option<String>,

    #[validate(length(min = 2, message = "Last name must be at least 2 characters long"))]
    pub last_name: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,


    pub phone: Option<String>,
    pub password: String
}
