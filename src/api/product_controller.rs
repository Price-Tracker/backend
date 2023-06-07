use crate::middlewares::jwt_middleware::TokenClaims;
use crate::models::product::ProductFilter;
use crate::models::response::ResponseBody;
use crate::services::product_service;
use actix_web::{delete, get, put, web, HttpResponse, Result};
use deadpool_diesel::postgres::Pool;

#[utoipa::path(
    params(ProductFilter),
    responses(
        (status = 200, description = "Got a product list", body = ResponseVecProduct),
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
        (status = 200, description = "Got a product by id", body = ResponseProduct),
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
        (status = 200, description = "Got a product by product store id", body = ResponseProductStore),
        (status = 400, description = "Unknown error"),
    ),
        context_path = "/api"
)]
#[get("/product/byStoreId/{id}")]
pub async fn product_by_product_store_id(
    product_store_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match product_service::product_by_product_store_id(product_store_id, &pool).await {
        Ok(product_dto) => Ok(HttpResponse::Ok().json(ResponseBody::new("success", product_dto))),
        Err(err) => Ok(err.response()),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got a product subscription", body = ResponseProductSubscription),
        (status = 400, description = "Unknown error"),
    ),
        context_path = "/api"
)]
#[get("/product/{id}/subscription")]
pub async fn get_product_subscription(
    product_id: web::Path<i32>,
    token_claims: TokenClaims,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match product_service::get_product_subscription(product_id, token_claims, &pool).await {
        Ok(product_subscription) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new("success", product_subscription)))
        }
        Err(err) => Ok(err.response()),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully subscribed to product"),
        (status = 400, description = "Unknown error"),
    ),
        context_path = "/api"
)]
#[put("/product/{id}/subscribe")]
pub async fn subscribe_to_product(
    product_id: web::Path<i32>,
    token_claims: TokenClaims,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match product_service::subscribe_to_product(product_id, token_claims, &pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new("success", ""))),
        Err(err) => Ok(err.response()),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully unsubscribed from product"),
        (status = 400, description = "Unknown error"),
    ),
        context_path = "/api"
)]
#[delete("/product/{id}/subscribe")]
pub async fn unsubscribe_from_product(
    product_id: web::Path<i32>,
    token_claims: TokenClaims,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match product_service::unsubscribe_from_product(product_id, token_claims, &pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new("success", ""))),
        Err(err) => Ok(err.response()),
    }
}
