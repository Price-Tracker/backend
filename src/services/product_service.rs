use crate::models::product::{Product, ProductDTO, ProductFilter};
use actix_web::web;
use actix_web::web::Data;
use deadpool_diesel::postgres::Pool;

pub async fn products(filters: web::Query<ProductFilter>, pool: &Data<Pool>) -> Vec<ProductDTO> {
    let conn = &pool.get().await.unwrap();

    conn.interact(|conn| Product::get_products_by_filter(conn, filters.0))
        .await
        .unwrap()
}
