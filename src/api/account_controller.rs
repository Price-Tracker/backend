use actix_jwt_auth_middleware::TokenSigner;
use actix_web::{Result, HttpResponse, web, post};
use deadpool_diesel::postgres::Pool;
use jwt_compact::alg::Ed25519;
use crate::models::response::ResponseBody;
use crate::models::user::{LoginDTO, UserClaims, UserDTO};
use crate::services::account_service;

#[post("/signup")]
pub async fn signup(user_dto: web::Json<UserDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match account_service::signup(user_dto.0, &pool).await {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, ""))),
        Err(err) => Ok(err.response()),
    }
}

#[post("/login")]
pub async fn login(login: web::Json<LoginDTO>, pool: web::Data<Pool>, cookie_signer: web::Data<TokenSigner<UserClaims, Ed25519>>) -> Result<HttpResponse> {
    match account_service::login(login.0, &pool).await {
        Ok(user_claims) => {
            Ok(HttpResponse::Ok()
                .cookie(cookie_signer.create_access_cookie(&user_claims).unwrap())
                .cookie(cookie_signer.create_refresh_cookie(&user_claims).unwrap())
                .finish()
            )
        }
        Err(err) => { Ok(err.response()) }
    }
}