use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashSet;

use crate::schema::stores::{self, dsl::*};

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
    pub fn get_store_name_by_id(conn: &mut PgConnection, store_id: i32) -> QueryResult<String> {
        stores
            .select(name)
            .filter(id.eq(store_id))
            .first::<String>(conn)
    }
}
