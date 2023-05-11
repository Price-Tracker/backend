use actix_web::{get, Error, HttpResponse, web};
use deadpool_diesel::postgres::Pool;
use diesel::{IntoSql, RunQueryDsl, select};
use diesel::sql_types::Text;
use crate::errors::MyError;

#[get("/ping")]
pub async fn ping(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let conn = db_pool.get().await.map_err(MyError::PoolError)?;

    let result = conn.interact(|conn| {
        let query = select("Hello world!".into_sql::<Text>());
        query.get_result::<String>(conn)
    }).await;

    if let Ok(..) = result {
        Ok(HttpResponse::Ok().json("pong!"))
    } else {
        Err(MyError::UnknownError.into())
    }
}