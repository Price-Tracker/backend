use crate::models::response::ResponseBody;
use crate::services::category_service;
use actix_web::{get, web, HttpResponse, Result};
use deadpool_diesel::postgres::Pool;

#[utoipa::path(
    responses(
        (status = 200, description = "Got a category list", body = String),
        (status = 400, description = "Unknown error"),
    ),
        context_path = "/api"
    )]
#[get("/categories")]
pub async fn categories(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match category_service::categories(&pool).await {
        Ok(categories) => Ok(HttpResponse::Ok().json(ResponseBody::new("success", categories))),
        Err(err) => Ok(err.response()),
    }
}
