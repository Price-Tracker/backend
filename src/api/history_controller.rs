use crate::middlewares::jwt_middleware::TokenClaims;
use crate::models::response::ResponseBody;
use crate::models::user::HistoryDTO;
use crate::services::history_service;
use actix_web::{get, post, web, HttpResponse, Result};
use deadpool_diesel::postgres::Pool;

#[utoipa::path(
    request_body = HistoryDTO,
    responses(
        (status = 200, description = "Successfully added to history"),
        (status = 400, description = "Unknown error"),
    ),
    context_path = "/api"
)]
#[post("/history")]
pub async fn add_to_history(
    history_dto: web::Json<HistoryDTO>,
    token_claims: TokenClaims,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match history_service::add_to_history(history_dto, token_claims, &pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new("success", ""))),
        Err(err) => Ok(err.response()),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got a history list", body = ResponseVecHistory),
        (status = 400, description = "Unknown error"),
    ),
    context_path = "/api"
)]
#[get("/history")]
pub async fn get_history(token_claims: TokenClaims, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match history_service::get_history(token_claims, &pool).await {
        Ok(history) => Ok(HttpResponse::Ok().json(ResponseBody::new("success", history))),
        Err(err) => Ok(err.response()),
    }
}
