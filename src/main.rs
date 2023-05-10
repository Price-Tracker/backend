mod schema;

use std::env;
use actix_web::{get, HttpResponse, middleware, Responder};
use diesel::{Connection, PgConnection};
use diesel::prelude::*;
use dotenvy::dotenv;

#[derive(Queryable)]
pub struct Test {
    pub id: i32,
    pub is_done: bool
}

#[get("/ping")]
async fn ping() -> impl Responder {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    if let Ok(mut connection) = PgConnection::establish(&database_url) {
        let connection = &mut connection;
        use self::schema::test::dsl::*;

        let results = test
            .load::<Test>(connection)
            .expect("Fail");

        HttpResponse::Ok().body(format!("Pong! Got results: {}", results.len()))
    } else {
        HttpResponse::BadGateway().body("Failed to connect to database!")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting server at http://127.0.0.1:8080");

    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(ping)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
