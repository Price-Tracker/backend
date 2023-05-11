use actix_web::{Result, HttpResponse, web, post};
use deadpool_diesel::postgres::Pool;
use crate::models::response::ResponseBody;
use crate::models::user::UserDTO;
use crate::services::account_service;

#[post("/signup")]
pub async fn signup(user_dto: web::Json<UserDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match account_service::signup(user_dto.0, &pool).await {
        Ok(message) => Ok(HttpResponse::Ok().json(ResponseBody::new(&message, ""))),
        Err(err) => Ok(err.response()),
    }
}