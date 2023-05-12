use actix_web::web::Data;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use chrono::{Duration, NaiveDate, Utc};
use crate::schema::users::{self, dsl::*};
use diesel::prelude::*;
use jsonwebtoken::EncodingKey;
use log::info;
use serde::{Serialize, Deserialize};
use crate::config::app::Config;
use crate::middlewares::jwt_middleware::TokenClaims;
use crate::models::user_tokens::UserTokensDTO;

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
    pub fn signup(conn: &mut PgConnection, user: UserDTO) -> Result<String, String> {
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

    pub fn login(conn: &mut PgConnection, login_cred: LoginDTO, config: Data<Config>) -> Result<UserTokensDTO, String> {
        if let Ok(fetched_user) = users
            .filter(login.eq(&login_cred.login_or_email))
            .or_filter(email.eq(&login_cred.login_or_email))
            .get_result::<User>(conn) {
            info!("Found user with id {}", fetched_user.id);
            if let Ok(parsed_hash) = PasswordHash::new(&fetched_user.password) {
                if Argon2::default().verify_password(login_cred.password.as_bytes(), &parsed_hash).is_ok() {
                    info!("Password for user id {} is right", fetched_user.id);

                    let now = Utc::now();

                    let token_claims = TokenClaims {
                        sub: fetched_user.id.to_string(),
                        iat: now.timestamp() as usize,
                        exp: (now + Duration::seconds(config.jwt_expires_in_secs as i64)).timestamp() as usize,
                        nickname: fetched_user.nickname,
                        login: fetched_user.login,
                        roles: vec![]
                    };

                    let access_token = jsonwebtoken::encode(
                        &jsonwebtoken::Header::default(),
                        &token_claims,
                        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
                    ).unwrap();

                    let user_tokens = UserTokensDTO {
                        access_token,
                        // TODO: implement refresh token
                        refresh_token: String::new()
                    };

                    return Ok(user_tokens);
                } else {
                    info!("Password for user id {} is wrong", fetched_user.id);
                }
            }
        } else {
            info!("Can't find user with login or email {}", login_cred.login_or_email);
        }
        Err("Login, email or password is wrong!".to_string())
    }

    pub fn find_user_by_login(conn: &mut PgConnection, _login: &str) -> QueryResult<User> {
        users.filter(login.eq(_login)).get_result::<User>(conn)
    }

    pub fn find_user_by_email(conn: &mut PgConnection, _email: &str) -> QueryResult<User> {
        users.filter(email.eq(_email)).get_result::<User>(conn)
    }
}