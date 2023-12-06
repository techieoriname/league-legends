// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    challenges (id) {
        id -> Uuid,
        challenger_id -> Uuid,
        opponent_id -> Uuid,
        match_date -> Timestamp,
        #[max_length = 50]
        status -> Varchar,
        result -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    leagues (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        creator_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    teams (id) {
        id -> Uuid,
        league_id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> Uuid,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        middle_name -> Nullable<Varchar>,
        #[max_length = 255]
        last_name -> Nullable<Varchar>,
        date_of_birth -> Nullable<Date>,
        #[max_length = 50]
        gender -> Nullable<Varchar>,
        #[max_length = 20]
        phone -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        role -> Nullable<UserRole>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 255]
        fpl_team_id -> Nullable<Varchar>,
    }
}

diesel::joinable!(leagues -> users (creator_id));
diesel::joinable!(teams -> leagues (league_id));
diesel::joinable!(teams -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    challenges,
    leagues,
    teams,
    users,
);
