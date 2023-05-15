use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::sql_types::Float;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use crate::schema::product_store_prices::{self, dsl::*};
use crate::schema::products::{self, dsl::*};

sql_function!(fn array_agg(x: Float) -> Array<Float>);

#[derive(Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub picture_url: Option<String>,
    pub created_date: NaiveDate,
    pub updated_date: NaiveDate,
}

#[derive(Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(Product))]
#[diesel(table_name = product_store_prices)]
pub struct ProductStorePrice {
    pub store_id: i32,
    pub product_id: i32,
    pub price: f32,
    pub created_date: NaiveDate,
    pub id: i32,
}

#[derive(Deserialize)]
pub struct ProductFilter {
    category_id: Option<i32>,
    min_price: Option<f32>,
    max_price: Option<f32>,
}

#[derive(Serialize)]
pub struct ProductDTO {
    product: Product,
    prices: Vec<ProductStorePrice>,
    min_price: Option<f32>,
    max_price: Option<f32>,
}

impl Product {
    pub fn get_products_by_filter(
        conn: &mut PgConnection,
        filter: ProductFilter,
    ) -> Vec<ProductDTO> {
        let mut products_query = products::table.select(Product::as_select()).into_boxed();

        if let Some(_category_id) = filter.category_id {
            products_query = products_query.filter(category_id.eq(_category_id));
        }

        let mut product_ids_with_prices = product_store_prices
            .select(product_id)
            .group_by(product_id)
            .into_boxed();

        if let Some(min_price) = filter.min_price {
            product_ids_with_prices = product_ids_with_prices.filter(price.ge(min_price))
        }

        if let Some(max_price) = filter.max_price {
            product_ids_with_prices = product_ids_with_prices.filter(price.le(max_price))
        }

        products_query = products_query.filter(products::id.eq_any(product_ids_with_prices));

        let filtered_products = products_query.load::<Product>(conn).unwrap();

        let prices_query = ProductStorePrice::belonging_to(&filtered_products)
            .select(ProductStorePrice::as_select())
            .into_boxed();

        let filtered_prices = prices_query.load::<ProductStorePrice>(conn).unwrap();

        filtered_prices
            .grouped_by(&filtered_products)
            .into_iter()
            .zip(filtered_products)
            .map(|(prices, product)| {
                let min_price = prices
                    .iter()
                    .min_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal))
                    .map(|p| p.price);
                let max_price = prices
                    .iter()
                    .max_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal))
                    .map(|p| p.price);

                ProductDTO {
                    product,
                    prices,
                    min_price,
                    max_price,
                }
            })
            .collect::<Vec<ProductDTO>>()
    }
}
