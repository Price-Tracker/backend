mod schema;
mod handlers;
mod errors;

use std::env;
use actix_web::{middleware, web};
use deadpool_diesel::postgres::{Manager, Pool};
use deadpool_diesel::Runtime;
use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use log::info;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

async fn get_connection_pool(db_url: String) -> Pool {
    info!("Creating connections pool...");
    let manager = Manager::new(db_url, Runtime::Tokio1);
    let pool = Pool::builder(manager)
        .max_size(32)
        .build()
        .unwrap();

    info!("Connection pool has been created successfully.");
    pool
}

async fn run_migrations(db_url: &String) {
    info!("Preparing to run db migrations...");
    let mut conn =
        PgConnection::establish(db_url).unwrap_or_else(|e| panic!("Error connecting to {db_url}: {e}"));
    let _ = &mut conn
        .run_pending_migrations(MIGRATIONS)
        .unwrap_or_else(|e| panic!("Couldn't run DB Migrations: {e}"));
    info!("Database migrations complete.");
}

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

    run_migrations(&db_url).await;
    let pool = get_connection_pool(db_url).await;

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
