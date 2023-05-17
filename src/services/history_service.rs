use crate::errors::ServiceError;
use crate::middlewares::jwt_middleware::TokenClaims;
use crate::models::user::User;
use actix_web::http::StatusCode;
use actix_web::web;
use deadpool_diesel::postgres::Pool;

pub async fn add_product_to_history(
    product_id: web::Path<i32>,
    token_claims: TokenClaims,
    pool: &web::Data<Pool>,
) -> Result<String, ServiceError> {
    let conn = &pool.get().await.unwrap();

    conn.interact(move |conn| {
        let user = User::find_user_by_login(conn, &token_claims.login);

        match user {
            Ok(user) => {
                match User::add_product_to_history(conn, user.id, product_id.into_inner()) {
                    Ok(_) => Ok("".to_string()),
                    Err(message) => Err(ServiceError::new(
                        StatusCode::BAD_REQUEST,
                        message.to_string(),
                    )),
                }
            }
            Err(message) => Err(ServiceError::new(
                StatusCode::BAD_REQUEST,
                message.to_string(),
            )),
        }
    })
    .await
    .unwrap()
}
