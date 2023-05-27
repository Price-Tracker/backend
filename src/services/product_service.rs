use crate::errors::ServiceError;
use crate::middlewares::jwt_middleware::TokenClaims;
use crate::models::product::{Product, ProductDTO, ProductFilter};
use crate::models::user::{User, UserSubscribedProductDTO};
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;

pub async fn products(
    filters: web::Query<ProductFilter>,
    pool: &Data<Pool>,
) -> Result<Vec<ProductDTO>, ServiceError> {
    let conn = &pool.get().await.unwrap();

    conn.interact(
        |conn| match Product::get_products_by_filter(conn, filters.0) {
            Ok(products) => Ok(products),
            Err(message) => Err(ServiceError::new(
                StatusCode::BAD_REQUEST,
                message.to_string(),
            )),
        },
    )
    .await
    .unwrap()
}

pub async fn product(
    product_id: web::Path<i32>,
    pool: &Data<Pool>,
) -> Result<ProductDTO, ServiceError> {
    let conn = &pool.get().await.unwrap();

    conn.interact(
        |conn| match Product::get_product(conn, product_id.into_inner()) {
            Ok(product) => Ok(product),
            Err(message) => Err(ServiceError::new(
                StatusCode::BAD_REQUEST,
                message.to_string(),
            )),
        },
    )
    .await
    .unwrap()
}

pub async fn get_product_subscription(
    product_id: web::Path<i32>,
    token_claims: TokenClaims,
    pool: &Data<Pool>,
) -> Result<UserSubscribedProductDTO, ServiceError> {
    let conn = &pool.get().await.unwrap();

    conn.interact(move |conn| {
        let user = User::find_user_by_login(conn, &token_claims.login);

        match user {
            Ok(user) => Ok(Product::get_product_subscription(
                conn,
                user.id,
                product_id.into_inner(),
            )),
            Err(message) => Err(ServiceError::new(
                StatusCode::BAD_REQUEST,
                message.to_string(),
            )),
        }
    })
    .await
    .unwrap()
}

pub async fn subscribe_to_product(
    product_id: web::Path<i32>,
    token_claims: TokenClaims,
    pool: &Data<Pool>,
) -> Result<(), ServiceError> {
    let conn = &pool.get().await.unwrap();

    conn.interact(move |conn| {
        let user = User::find_user_by_login(conn, &token_claims.login);

        match user {
            Ok(user) => match Product::subscribe_to_product(conn, user.id, product_id.into_inner())
            {
                Ok(code) => {
                    if code == 0 {
                        return Err(ServiceError::new(
                            StatusCode::BAD_REQUEST,
                            "Subscription already exists".to_string(),
                        ));
                    }
                    Ok(())
                }
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

pub async fn unsubscribe_from_product(
    product_id: web::Path<i32>,
    token_claims: TokenClaims,
    pool: &Data<Pool>,
) -> Result<(), ServiceError> {
    let conn = &pool.get().await.unwrap();

    conn.interact(move |conn| {
        let user = User::find_user_by_login(conn, &token_claims.login);

        match user {
            Ok(user) => {
                match Product::unsubscribe_from_product(conn, user.id, product_id.into_inner()) {
                    Ok(code) => {
                        if code == 0 {
                            return Err(ServiceError::new(
                                StatusCode::BAD_REQUEST,
                                "Subscription not found".to_string(),
                            ));
                        }
                        Ok(())
                    }
                    Err(message) => Err(ServiceError::new(
                        StatusCode::BAD_REQUEST,
                        message.to_string(),
                    )),
                }
            }
            Err(message) => Err(ServiceError::new(
                StatusCode::BAD_REQUEST,
                message.to_string(),
            )),
        }
    })
    .await
    .unwrap()
}
