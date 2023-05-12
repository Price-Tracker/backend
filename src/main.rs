mod api;
mod config;
mod errors;
mod models;
mod schema;
mod services;

use actix_web::{App, HttpServer, middleware, web};
use dotenvy::dotenv;
use log::info;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set");
    let app_url = format!("{}:{}", app_host, app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = config::db::get_connection_pool(db_url).await;
    config::db::run_migrations(pool.clone()).await;

    info!("Starting server at http://{app_url}");

    HttpServer::new(move || App::new()
        .wrap(middleware::Logger::default())
        .wrap(config::app::get_cors())
        .app_data(web::Data::new(pool.clone()))
        .configure(config::app::configure_services)
    )
        .bind(app_url)?
        .run()
        .await
}
