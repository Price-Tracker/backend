use crate::middlewares::jwt_middleware::TokenClaims;
use crate::models::response::ResponseBody;
use crate::services::history_service;
use actix_web::{post, web, HttpResponse, Result};
use deadpool_diesel::postgres::Pool;

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
    match history_service::add_product_to_history(product_id, token_claims, &pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new("success", ""))),
        Err(err) => Ok(err.response()),
    }
}
