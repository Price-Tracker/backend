use deadpool_diesel::postgres::{Manager, Pool};
use deadpool_diesel::Runtime;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub async fn get_connection_pool(db_url: &str) -> Pool {
    info!("Creating connections pool...");
    let manager = Manager::new(db_url, Runtime::Tokio1);
    let pool = Pool::builder(manager).max_size(32).build().unwrap();

    info!("Connection pool has been created successfully.");
    pool
}

pub async fn run_migrations(pool: Pool) {
    info!("Preparing to run db migrations...");
    let conn = pool
        .get()
        .await
        .expect("Failed to get connection from the pool.");
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();

    info!("Database migrations complete.");
}
