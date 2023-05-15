mod api;
mod config;
mod errors;
mod middlewares;
mod models;
mod schema;
mod services;

use crate::config::app::Config;
use actix_web::{middleware, web, App, HttpServer};
use dotenvy::dotenv;
use log::info;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::init();

    let pool = config::db::get_connection_pool(&config.database_url).await;
    config::db::run_migrations(pool.clone()).await;

    let app_url = config.app_url.clone();
    info!("Starting server at http://{}", app_url);

    let openapi = config::app::get_openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(config::app::get_cors())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(
                SwaggerUi::new("/api/swagger-ui/{_:.*}")
                    .url("/api/api-docs/openapi.json", openapi.clone()),
            )
            .configure(config::app::configure_services)
    })
    .bind(app_url)?
    .run()
    .await
}
