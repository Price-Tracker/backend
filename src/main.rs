mod schema;
mod handlers;
mod errors;

use std::env;
use actix_web::{middleware, web};
use deadpool_diesel::postgres::{Manager, Pool};
use deadpool_diesel::Runtime;
use dotenvy::dotenv;

async fn get_connection_pool() -> Pool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = Manager::new(database_url, Runtime::Tokio1);
    Pool::builder(manager)
        .max_size(32)
        .build()
        .unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    dotenv().ok();

    let pool = get_connection_pool().await;

    let ip = env::var("IP")
        .unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("Failed to parse PORT env");

    log::info!("Starting server at http://{ip}:{port}");

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
