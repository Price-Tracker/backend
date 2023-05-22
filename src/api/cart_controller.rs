use crate::middlewares::jwt_middleware::TokenClaims;
use crate::models::response::ResponseBody;
use crate::models::user::UserShoppingCartDTO;
use crate::services::cart_service;
use actix_web::{get, put, web, HttpResponse, Result};
use deadpool_diesel::postgres::Pool;

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully added to cart"),
        (status = 400, description = "Unknown error"),
    ),
        context_path = "/api/cart"
    )]
#[put("/add")]
pub async fn add_to_cart(
    cart_dto: web::Json<UserShoppingCartDTO>,
    token_claims: TokenClaims,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match cart_service::add_to_cart(cart_dto, token_claims, &pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new("success", ""))),
        Err(err) => Ok(err.response()),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got a cart list", body = UserShoppingCartDTO),
        (status = 400, description = "Unknown error"),
    ),
        context_path = "/api/cart"
    )]
#[get("")]
pub async fn get_cart(token_claims: TokenClaims, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match cart_service::get_cart(token_claims, &pool).await {
        Ok(cart) => Ok(HttpResponse::Ok().json(ResponseBody::new("success", cart))),
        Err(err) => Ok(err.response()),
    }
}
