use deadpool_diesel::postgres::{Manager, Pool};
use deadpool_diesel::Runtime;
use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub async fn get_connection_pool(db_url: String) -> Pool {
    info!("Creating connections pool...");
    let manager = Manager::new(db_url, Runtime::Tokio1);
    let pool = Pool::builder(manager)
        .max_size(32)
        .build()
        .unwrap();

    info!("Connection pool has been created successfully.");
    pool
}

pub async fn run_migrations(db_url: &String) {
    info!("Preparing to run db migrations...");
    let mut conn =
        PgConnection::establish(db_url).unwrap_or_else(|e| panic!("Error connecting to {db_url}: {e}"));
    let _ = &mut conn
        .run_pending_migrations(MIGRATIONS)
        .unwrap_or_else(|e| panic!("Couldn't run DB Migrations: {e}"));
    info!("Database migrations complete.");
}