use crate::models::product::ProductFilter;
use crate::models::response::ResponseBody;
use crate::services::product_service;
use actix_web::{get, web, HttpResponse, Result};
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
