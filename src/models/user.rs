use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use chrono::{NaiveDate};
use crate::schema::users::{self, dsl::*};
use diesel::prelude::*;
use log::info;
use serde::{Serialize, Deserialize};

#[derive(Identifiable, Queryable)]
pub struct User {
    pub id: i32,
    pub nickname: Option<String>,
    pub login: String,
    pub email: String,
    pub password: String,
    pub created_date: NaiveDate,
    pub updated_date: NaiveDate,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub login: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub login_or_email: String,
    pub password: String,
}

impl User {
    pub fn signup(user: UserDTO, conn: &mut PgConnection) -> Result<String, String> {
        if Self::find_user_by_login(conn, &user.login).is_err()
            && Self::find_user_by_email(conn, &user.email).is_err() {
            let salt = SaltString::generate(&mut OsRng);
            let hashed_password = Argon2::default()
                .hash_password(user.password.as_bytes(), &salt)
                .expect("Error while hashing password")
                .to_string();

            info!("Hashed password: {}, len: {}", hashed_password, hashed_password.len());

            let user = UserDTO {
                password: hashed_password,
                ..user
            };

            diesel::insert_into(users).values(user).execute(conn).unwrap();

            Ok("Signup successfully".to_string())
        } else {
            Err(format!("Login '{}' or Email '{}' is already registered", &user.login, &user.email))
        }
    }

    pub fn find_user_by_login(conn: &mut PgConnection, _login: &str) -> QueryResult<User> {
        users.filter(login.eq(_login)).get_result::<User>(conn)
    }

    pub fn find_user_by_email(conn: &mut PgConnection, _email: &str) -> QueryResult<User> {
        users.filter(email.eq(_email)).get_result::<User>(conn)
    }
}