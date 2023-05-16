use crate::errors::ServiceError;
use crate::models::product::{Product, ProductDTO, ProductFilter};
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
