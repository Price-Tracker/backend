use actix_web::{HttpResponse, ResponseError};
use deadpool_diesel::{PoolError};
use derive_more::{Display, From};

#[derive(Display, From, Debug)]
pub enum MyError {
    NotFound,
    PoolError(PoolError),
    UnknownError
}
impl std::error::Error for MyError {}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}