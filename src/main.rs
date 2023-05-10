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
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let ip = env::var("IP")
        .unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("Failed to parse PORT env");

    log::info!("Starting server at http://{ip}:{port}");

    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(ping)
    })
        .bind((ip, port))?
        .run()
        .await
}
