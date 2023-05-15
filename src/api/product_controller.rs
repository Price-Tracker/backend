use actix_web::{get, web, HttpResponse, Result};
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
    let result = product_service::products(filters, &pool).await;
    Ok(HttpResponse::Ok().json(ResponseBody::new("success", result)))
}
