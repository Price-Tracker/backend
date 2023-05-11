mod schema;
mod handlers;
mod errors;
mod config;

use std::env;
use actix_web::{middleware, web};
use dotenvy::dotenv;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    dotenv().ok();

    let ip = env::var("IP")
        .unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("Failed to parse PORT env");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    config::db::run_migrations(&db_url).await;
    let pool = config::db::get_connection_pool(db_url).await;

    info!("Starting server at http://{ip}:{port}");

    use actix_web::{App, HttpServer};

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::ping)
    })
        .bind((ip, port))?
        .run()
        .await
}
