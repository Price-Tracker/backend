use crate::errors::ServiceError;
use crate::models::category::Category;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;

pub async fn categories(pool: &Data<Pool>) -> Result<Vec<Category>, ServiceError> {
    let conn = &pool.get().await.unwrap();

    conn.interact(|conn| match Category::get_categories(conn) {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            message.to_string(),
        )),
    })
    .await
    .unwrap()
}
