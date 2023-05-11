use actix_web::http::StatusCode;
use actix_web::web;
use deadpool_diesel::postgres::Pool;
use crate::errors::ServiceError;
use crate::models::user::{LoginDTO, User, UserClaims, UserDTO};

pub async fn signup(user: UserDTO, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
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

pub async fn login(login: LoginDTO, pool: &web::Data<Pool>) -> Result<UserClaims, ServiceError> {
    let conn = &pool.get().await.unwrap();

    conn.interact(|conn|
        match User::login(conn, login) {
            Ok(user_claims) => Ok(user_claims),
            Err(message) => Err(ServiceError::new(StatusCode::UNAUTHORIZED, message)),
        }
    )
        .await
        .unwrap()
}