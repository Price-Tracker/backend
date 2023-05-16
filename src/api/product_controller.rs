use crate::middlewares::jwt_middleware::TokenClaims;
use actix_web::{get, post, web, HttpResponse, Result};
use deadpool_diesel::postgres::Pool;

use crate::models::product::ProductFilter;
use crate::models::response::ResponseBody;
use crate::services::product_service;

#[utoipa::path(
    responses(
        (status = 200, description = "Got a product list", body = String),
        (status = 400, description = "Unknown error"),
    ),
        context_path = "/api"
    )]
#[get("/products")]
pub async fn products(
    filters: web::Query<ProductFilter>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match product_service::products(filters, &pool).await {
        Ok(products) => Ok(HttpResponse::Ok().json(ResponseBody::new("success", products))),
        Err(err) => Ok(err.response()),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got a product", body = String),
        (status = 400, description = "Unknown error"),
    ),
        context_path = "/api"
)]
#[get("/product/{id}")]
pub async fn product(product_id: web::Path<i32>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match product_service::product(product_id, &pool).await {
        Ok(product) => Ok(HttpResponse::Ok().json(ResponseBody::new("success", product))),
        Err(err) => Ok(err.response()),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got a product", body = String),
        (status = 400, description = "Unknown error"),
    ),
    context_path = "/api"
)]
#[post("/history/product/{id}")]
pub async fn add_product_to_history(
    product_id: web::Path<i32>,
    token_claims: TokenClaims,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match product_service::add_product_to_history(product_id, token_claims, &pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new("success", ""))),
        Err(err) => Ok(err.response()),
    }
}
