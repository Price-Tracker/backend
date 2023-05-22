use crate::schema::product_stores::store_id;
use crate::schema::product_stores::{self, dsl::*};
use crate::schema::stores::{self, dsl::*};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = stores)]
pub struct Store {
    pub id: i32,
    pub retail_chain_id: i32,
    pub country_id: i32,
    pub region_id: i32,
    pub city_id: i32,
    pub name: String,
    pub location: String,
    pub latitude: f32,
    pub longitude: f32,
    pub created_date: NaiveDateTime,
}

impl Store {
    pub fn get_store_name_by_id(conn: &mut PgConnection, _store_id: i32) -> QueryResult<String> {
        stores
            .select(name)
            .filter(stores::id.eq(_store_id))
            .first::<String>(conn)
    }

    pub fn get_store_id_by_product_store_id(
        conn: &mut PgConnection,
        product_store_id: i32,
    ) -> QueryResult<i32> {
        product_stores
            .select(store_id)
            .filter(product_stores::id.eq(product_store_id))
            .first::<i32>(conn)
    }
}
