use std::env;
use actix_cors::Cors;
use actix_web::{http, web};
use utoipa::{OpenApi, openapi};
use crate::models::user::{LoginDTO, UserDTO};
use crate::models::user_tokens::{UserTokensDTO, UserRefreshTokenDTO};
use crate::api::*;
use crate::models::response::ResponseTokens;

#[derive(Debug, Clone)]
pub struct Config {
    pub app_url: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in_secs: i32,
}

impl Config {
    pub fn init() -> Config {
        let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
        let app_port = env::var("APP_PORT").expect("APP_PORT must be set");
        let app_url = format!("{}:{}", app_host, app_port);
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in_secs = env::var("JWT_EXPIRES_IN_SECS")
            .expect("JWT_EXPIRES_IN_SECS must be set")
            .parse::<i32>().unwrap();
        Config {
            app_url,
            database_url,
            jwt_secret,
            jwt_expires_in_secs,
        }
    }
}

pub fn get_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        // .allowed_origin_fn(|origin, _req_head| {
        //     origin.as_bytes().starts_with(b"https://infinity.tail1f457.ts.net")
        // })
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .max_age(3600)
        .supports_credentials()
}

pub fn get_openapi() -> openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            account_controller::signup,
            account_controller::login,
            account_controller::refresh_token,
            ping_controller::ping
        ),
        components(
            schemas(
                UserRefreshTokenDTO,
                UserTokensDTO,
                LoginDTO,
                UserDTO,
                ResponseTokens
            )
        )
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    openapi
}

pub fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/user")
                    .service(account_controller::signup)
                    .service(account_controller::login)
                    .service(account_controller::refresh_token)
            )
            .service(ping_controller::ping)
    );
}