use crate::api::*;
use crate::models::category::Category;
use crate::models::product::{Product, ProductDTO, ProductStorePriceDTO};
use crate::models::response::{
    ResponsePasswordRequirements, ResponseProduct, ResponseTokens, ResponseVecCategory,
    ResponseVecHistory, ResponseVecProduct, ResponseVecShoppingCart,
};
use crate::models::user::HistoryWithProductDTO;
use crate::models::user::{HistoryDTO, LoginDTO, UserDTO, UserShoppingCartDTO};
use crate::models::user_tokens::{UserRefreshTokenDTO, UserTokensDTO};
use actix_cors::Cors;
use actix_web::web;
use std::env;
use utoipa::{openapi, OpenApi};

// Env var names
static APP_HOST: &str = "APP_HOST";
static APP_PORT: &str = "APP_PORT";
static DATABASE_URL: &str = "DATABASE_URL";
static JWT_SECRET: &str = "JWT_SECRET";
static JWT_EXPIRES_IN_SECS: &str = "JWT_EXPIRES_IN_SECS";

// Env defaults
static APP_HOST_DEFAULT: &str = "0.0.0.0";

#[derive(Debug, Clone)]
pub struct Config {
    pub app_url: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in_secs: i32,
}

impl Config {
    pub fn init() -> Config {
        let app_host = env::var(APP_HOST).unwrap_or_else(|_| APP_HOST_DEFAULT.to_string());
        let app_port = env::var(APP_PORT).expect(&format!("{APP_PORT} must be set"));
        let app_url = format!("{}:{}", app_host, app_port);
        let database_url = env::var(DATABASE_URL).expect(&format!("{DATABASE_URL} must be set"));
        let jwt_secret = env::var(JWT_SECRET).expect(&format!("{JWT_SECRET} must be set"));
        let jwt_expires_in_secs = env::var(JWT_EXPIRES_IN_SECS)
            .expect(&format!("{JWT_EXPIRES_IN_SECS} must be set"))
            .parse::<i32>()
            .unwrap();
        Config {
            app_url,
            database_url,
            jwt_secret,
            jwt_expires_in_secs,
        }
    }
}

pub fn get_cors() -> Cors {
    Cors::permissive()
}

pub fn get_openapi() -> openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            account_controller::login,
            account_controller::refresh_token,
            account_controller::signup,
            account_controller::password_requirements,
            cart_controller::add_to_cart,
            cart_controller::get_cart,
            category_controller::categories,
            history_controller::add_to_history,
            history_controller::get_history,
            ping_controller::ping,
            product_controller::product,
            product_controller::products,
        ),
        components(schemas(
            Category,
            HistoryDTO,
            HistoryWithProductDTO,
            LoginDTO,
            Product,
            ProductDTO,
            ProductStorePriceDTO,
            ResponseProduct,
            ResponseTokens,
            ResponsePasswordRequirements,
            ResponseVecCategory,
            ResponseVecHistory,
            ResponseVecProduct,
            ResponseVecShoppingCart,
            UserDTO,
            UserRefreshTokenDTO,
            UserShoppingCartDTO,
            UserTokensDTO,
        ))
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
                    .service(account_controller::password_requirements),
            )
            .service(
                web::scope("/cart")
                    .service(cart_controller::add_to_cart)
                    .service(cart_controller::get_cart),
            )
            .service(ping_controller::ping)
            .service(category_controller::categories)
            .service(product_controller::products)
            .service(product_controller::product)
            .service(history_controller::add_to_history)
            .service(history_controller::get_history),
    );
}
