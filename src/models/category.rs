use crate::schema::categories::{self, dsl::*};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

impl Category {
    pub fn get_categories(conn: &mut PgConnection) -> QueryResult<Vec<Category>> {
        categories.get_results::<Category>(conn)
    }
}
