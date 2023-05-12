use actix_web::http::StatusCode;
use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;
use crate::config::app::Config;
use crate::errors::ServiceError;
use crate::models::user::{LoginDTO, User, UserDTO};
use crate::models::user_tokens::UserTokensDTO;

pub async fn signup(user: UserDTO, pool: &Data<Pool>) -> Result<String, ServiceError> {
    let conn = &pool.get().await.unwrap();

    conn.interact(|conn|
        match User::signup(conn, user) {
            Ok(message) => Ok(message),
            Err(message) => Err(ServiceError::new(StatusCode::BAD_REQUEST, message)),
        }
    )
        .await
        .unwrap()
}

pub async fn login(login: LoginDTO, pool: &Data<Pool>, config: Data<Config>) -> Result<UserTokensDTO, ServiceError> {
    let conn = &pool.get().await.unwrap();

    conn.interact(|conn|
        match User::login(conn, login, config) {
            Ok(user_claims) => Ok(user_claims),
            Err(message) => Err(ServiceError::new(StatusCode::UNAUTHORIZED, message)),
        }
    )
        .await
        .unwrap()
}