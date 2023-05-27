use crate::models::response::ResponseBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use deadpool_diesel::PoolError;
use derive_more::{Display, From};

#[derive(Display, From, Debug)]
pub enum MyError {
    NotFound,
    PoolError(PoolError),
    UnknownError,
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

pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                status: "error".to_string(),
                data: message,
            },
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(&self.body)
    }
}
