mod schema;
mod handlers;
mod errors;
mod config;

use std::env;
use actix_web::{App, HttpServer, middleware, web};
use dotenvy::dotenv;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set");
    let app_url = format!("{}:{}", app_host, app_port);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    config::db::run_migrations(&db_url).await;
    let pool = config::db::get_connection_pool(db_url).await;

    info!("Starting server at http://{app_url}");

    HttpServer::new(move || App::new()
        .wrap(middleware::Logger::default())
        .app_data(web::Data::new(pool.clone()))
        .service(handlers::ping)
    )
        .bind(app_url)?
        .run()
        .await
}
