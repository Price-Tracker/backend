use crate::errors::ServiceError;
use crate::middlewares::jwt_middleware::TokenClaims;
use crate::models::product::{Product, ProductDTO, ProductFilter};
use crate::models::user::User;
use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;
use diesel::QueryResult;

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

pub async fn add_product_to_history(
    product_id: web::Path<i32>,
    token_claims: TokenClaims,
    pool: &Data<Pool>,
) -> Result<String, ServiceError> {
    let conn = &pool.get().await.unwrap();

    conn.interact(move |conn| {
        let user = User::find_user_by_login(conn, &token_claims.login);

        match user {
            Ok(user) => {
                match User::add_product_to_history(conn, user.id, product_id.into_inner()) {
                    Ok(_) => Ok("".to_string()),
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
