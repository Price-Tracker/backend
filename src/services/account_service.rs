use actix_web::http::StatusCode;
use actix_web::web;
use deadpool_diesel::postgres::Pool;
use crate::errors::ServiceError;
use crate::models::user::{User, UserDTO};

pub async fn signup(user: UserDTO, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    let conn = &pool.get().await.unwrap();

    conn.interact(|conn|
        match User::signup(user, conn) {
            Ok(message) => Ok(message),
            Err(message) => Err(ServiceError::new(StatusCode::BAD_REQUEST, message)),
        }
    )
        .await
        .unwrap()
}