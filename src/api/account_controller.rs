use actix_web::{Result, HttpResponse, web, post};
use deadpool_diesel::postgres::Pool;
use serde_json::json;
use crate::config::app::Config;
use crate::models::response::ResponseBody;
use crate::models::user::{LoginDTO, UserDTO};
use crate::models::user_tokens::UserRefreshTokenDTO;
use crate::services::account_service;

#[post("/signup")]
pub async fn signup(user_dto: web::Json<UserDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match account_service::signup(user_dto.0, &pool).await {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, ""))),
        Err(err) => Ok(err.response()),
    }
}

#[post("/login")]
pub async fn login(login: web::Json<LoginDTO>, pool: web::Data<Pool>, config: web::Data<Config>) -> Result<HttpResponse> {
    match account_service::login(login.0, &pool, config).await {
        Ok(tokens) => {
            Ok(HttpResponse::Ok()
                .json(json!({"status": "success", "tokens": tokens}))
            )
        }
        Err(err) => { Ok(err.response()) }
    }
}

#[post("/refresh-token")]
pub async fn refresh_token(user_refresh_token: web::Json<UserRefreshTokenDTO>, pool: web::Data<Pool>, config: web::Data<Config>) -> Result<HttpResponse> {
    match account_service::refresh_token(user_refresh_token.0, &pool, config).await {
        Ok(tokens) => {
            Ok(HttpResponse::Ok()
                .json(json!({"status": "success", "tokens": tokens}))
            )
        }
        Err(err) => { Ok(err.response()) }
    }
}