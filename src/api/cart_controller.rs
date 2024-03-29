use crate::middlewares::jwt_middleware::TokenClaims;
use crate::models::response::ResponseBody;
use crate::models::user::UserShoppingCartDTO;
use crate::services::cart_service;
use actix_web::{get, put, web, HttpResponse, Result};
use deadpool_diesel::postgres::Pool;

#[utoipa::path(
    request_body = UserShoppingCartDTO,
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
        (status = 200, description = "Got a cart list", body = ResponseVecShoppingCart),
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

#[utoipa::path(
    responses(
        (status = 200, description = "Got a cart total price", body = ResponseCartTotalPrice),
        (status = 400, description = "Unknown error"),
    ),
        context_path = "/api/cart"
    )]
#[get("/total")]
pub async fn get_cart_total_price(
    token_claims: TokenClaims,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match cart_service::get_cart_total_price(token_claims, &pool).await {
        Ok(total_price) => Ok(HttpResponse::Ok().json(ResponseBody::new("success", total_price))),
        Err(err) => Ok(err.response()),
    }
}
