use actix_web::http::StatusCode;
use actix_web::web;
use deadpool_diesel::postgres::Pool;

use crate::errors::ServiceError;
use crate::middlewares::jwt_middleware::TokenClaims;
use crate::models::user::{User, UserShoppingCartDTO};

pub async fn add_to_cart(
    cart_dto: web::Json<UserShoppingCartDTO>,
    token_claims: TokenClaims,
    pool: &web::Data<Pool>,
) -> Result<String, ServiceError> {
    let conn = &pool.get().await.unwrap();

    conn.interact(move |conn| {
        let user = User::find_user_by_login(conn, &token_claims.login);

        match user {
            Ok(user) => match User::add_to_cart(conn, user.id, cart_dto.0) {
                Ok(_) => Ok("".to_string()),
                Err(message) => Err(ServiceError::new(
                    StatusCode::BAD_REQUEST,
                    message.to_string(),
                )),
            },
            Err(message) => Err(ServiceError::new(
                StatusCode::BAD_REQUEST,
                message.to_string(),
            )),
        }
    })
    .await
    .unwrap()
}

pub async fn get_cart(
    token_claims: TokenClaims,
    pool: &web::Data<Pool>,
) -> Result<Vec<UserShoppingCartDTO>, ServiceError> {
    let conn = &pool.get().await.unwrap();

    conn.interact(move |conn| {
        let user = User::find_user_by_login(conn, &token_claims.login);

        match user {
            Ok(user) => match User::get_cart(conn, user.id) {
                Ok(user_cart) => Ok(user_cart),
                Err(message) => Err(ServiceError::new(
                    StatusCode::BAD_REQUEST,
                    message.to_string(),
                )),
            },
            Err(message) => Err(ServiceError::new(
                StatusCode::BAD_REQUEST,
                message.to_string(),
            )),
        }
    })
    .await
    .unwrap()
}
