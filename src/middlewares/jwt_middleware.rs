use core::fmt;
use std::future::{ready, Ready};

use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use crate::config::app::Config;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub nickname: Option<String>,
    pub login: String,
    pub roles: Vec<String>,
}

impl FromRequest for TokenClaims {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let config = req.app_data::<web::Data<Config>>().unwrap();

        let token = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .map(|h| h.to_str().unwrap().to_string());

        if token.is_none() {
            let json_error = ErrorResponse {
                status: "error".to_string(),
                message: "You are not logged in, please login first".to_string(),
            };
            return ready(Err(ErrorUnauthorized(json_error)));
        }

        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(config.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                let json_error = ErrorResponse {
                    status: "error".to_string(),
                    message: "Invalid access token".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };

        ready(Ok(claims))
    }
}
