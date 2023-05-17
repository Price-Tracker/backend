use crate::config::app::Config;
use crate::models::product::Product;
use crate::models::user_tokens::{UserRefreshTokenDTO, UserToken, UserTokensDTO};
use crate::schema::user_product_history::user_id;
use crate::schema::user_product_history::{self, dsl::*};
use crate::schema::users::{self, dsl::*};
use actix_web::web::Data;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::insert_into;
use diesel::prelude::*;
use log::info;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Identifiable, Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub nickname: Option<String>,
    pub login: String,
    pub email: String,
    pub password: String,
    pub created_date: NaiveDate,
    pub updated_date: NaiveDate,
}

#[derive(Queryable, Associations, Selectable, Insertable, Serialize)]
#[diesel(belongs_to(Product))]
#[diesel(table_name = user_product_history)]
pub struct UserProductHistory {
    pub user_id: i32,
    pub product_id: i32,
    pub created_date: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub login: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginDTO {
    pub login_or_email: String,
    pub password: String,
}

impl User {
    pub fn signup(conn: &mut PgConnection, user: UserDTO) -> Result<String, String> {
        if Self::find_user_by_login(conn, &user.login).is_err()
            && Self::find_user_by_email(conn, &user.email).is_err()
        {
            let salt = SaltString::generate(&mut OsRng);
            let hashed_password = Argon2::default()
                .hash_password(user.password.as_bytes(), &salt)
                .expect("Error while hashing password")
                .to_string();

            info!(
                "Hashed password: {}, len: {}",
                hashed_password,
                hashed_password.len()
            );

            let user = UserDTO {
                password: hashed_password,
                ..user
            };

            diesel::insert_into(users)
                .values(user)
                .execute(conn)
                .unwrap();

            Ok("Signup successfully".to_string())
        } else {
            Err(format!(
                "Login '{}' or Email '{}' is already registered",
                &user.login, &user.email
            ))
        }
    }

    pub fn login(
        conn: &mut PgConnection,
        login_cred: LoginDTO,
        config: Data<Config>,
    ) -> Result<UserTokensDTO, String> {
        if let Ok(fetched_user) = users
            .filter(login.eq(&login_cred.login_or_email))
            .or_filter(email.eq(&login_cred.login_or_email))
            .get_result::<User>(conn)
        {
            info!("Found user with id {}", fetched_user.id);
            if let Ok(parsed_hash) = PasswordHash::new(&fetched_user.password) {
                if Argon2::default()
                    .verify_password(login_cred.password.as_bytes(), &parsed_hash)
                    .is_ok()
                {
                    info!("Password for user id {} is right", fetched_user.id);

                    let access_token =
                        UserToken::generate_access_token(fetched_user.clone(), config);
                    let refresh_token =
                        UserToken::generate_refresh_token(conn, fetched_user.clone());

                    let user_tokens = UserTokensDTO {
                        access_token,
                        refresh_token,
                    };

                    return Ok(user_tokens);
                } else {
                    info!("Password for user id {} is wrong", fetched_user.id);
                }
            }
        } else {
            info!(
                "Can't find user with login or email {}",
                login_cred.login_or_email
            );
        }
        Err("Login, email or password is wrong!".to_string())
    }

    pub fn refresh_token(
        conn: &mut PgConnection,
        user_refresh_token: UserRefreshTokenDTO,
        config: Data<Config>,
    ) -> Result<UserTokensDTO, String> {
        if let Ok(user_token) =
            UserToken::find_refresh_token(conn, user_refresh_token.refresh_token.clone())
        {
            let user_tokens = UserToken::refresh_tokens(conn, user_token, config);

            Ok(user_tokens)
        } else {
            Err("Refresh token not found!".to_string())
        }
    }

    pub fn add_product_to_history(
        conn: &mut PgConnection,
        _user_id: i32,
        _product_id: i32,
    ) -> QueryResult<usize> {
        insert_into(user_product_history)
            .values((user_id.eq(_user_id), product_id.eq(_product_id)))
            .execute(conn)
    }

    pub fn find_user_by_id(conn: &mut PgConnection, _id: i32) -> QueryResult<User> {
        users.filter(users::id.eq(_id)).get_result::<User>(conn)
    }

    pub fn find_user_by_login(conn: &mut PgConnection, _login: &str) -> QueryResult<User> {
        users.filter(login.eq(_login)).get_result::<User>(conn)
    }

    pub fn find_user_by_email(conn: &mut PgConnection, _email: &str) -> QueryResult<User> {
        users.filter(email.eq(_email)).get_result::<User>(conn)
    }
}
