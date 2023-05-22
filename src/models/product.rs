use crate::models::store::Store;
use crate::schema::product_store_prices::{self, dsl::*};
use crate::schema::product_stores::{self, dsl::*};
use crate::schema::products::{self, dsl::*};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashSet;
use utoipa::{IntoParams, ToSchema};

#[derive(Queryable, Identifiable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub picture_url: Option<String>,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Selectable, Serialize, ToSchema)]
#[diesel(belongs_to(Store))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = product_stores)]
pub struct ProductStore {
    pub id: i32,
    pub store_id: i32,
    pub price_id: i32,
    pub product_id: i32,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(Product))]
#[diesel(table_name = product_store_prices)]
pub struct ProductStorePrice {
    pub store_id: i32,
    pub product_id: i32,
    pub price: f32,
    pub created_date: NaiveDateTime,
    pub id: i32,
}

#[derive(Deserialize, IntoParams)]
pub struct ProductFilter {
    category_id: Option<i32>,
    min_price: Option<f32>,
    max_price: Option<f32>,
}

#[derive(Serialize, ToSchema)]
pub struct ProductStorePriceDTO {
    pub store_id: i32,
    pub store_name: String,
    pub price: f32,
}

// TODO: Replace Product struct
#[derive(Serialize, ToSchema)]
pub struct ProductDTO {
    product: Product,
    prices: Vec<ProductStorePriceDTO>,
    min_price: Option<f32>,
    max_price: Option<f32>,
}

impl Product {
    fn find_latest_price(
        conn: &mut PgConnection,
        _store_id: i32,
        _product_id: i32,
    ) -> Option<ProductStorePrice> {
        product_store_prices
            .filter(product_store_prices::store_id.eq(_store_id))
            .filter(product_store_prices::product_id.eq(_product_id))
            .order(product_store_prices::created_date.desc())
            .first::<ProductStorePrice>(conn)
            .optional()
            .expect("Error loading latest price")
    }

    fn map_product_and_prices(
        conn: &mut PgConnection,
        prices: Vec<ProductStorePrice>,
        product: Product,
    ) -> ProductDTO {
        let unique_store_ids: HashSet<i32> = prices.iter().map(|p| p.store_id).collect();
        let latest_prices: Vec<ProductStorePriceDTO> = unique_store_ids
            .iter()
            .filter_map(|_store_id| Self::find_latest_price(conn, *_store_id, product.id))
            .collect::<Vec<ProductStorePrice>>()
            .into_iter() // conn cannot be accessed simultaneously
            .map(|_price| ProductStorePriceDTO {
                store_id: _price.store_id,
                store_name: Store::get_store_name_by_id(conn, _price.store_id).unwrap(),
                price: _price.price,
            })
            .collect();

        let min_price = latest_prices
            .iter()
            .min_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal))
            .map(|p| p.price);
        let max_price = latest_prices
            .iter()
            .max_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal))
            .map(|p| p.price);

        ProductDTO {
            product,
            prices: latest_prices,
            min_price,
            max_price,
        }
    }

    pub fn get_products_by_filter(
        conn: &mut PgConnection,
        filter: ProductFilter,
    ) -> QueryResult<Vec<ProductDTO>> {
        let mut products_query = products::table.select(Product::as_select()).into_boxed();

        if let Some(_category_id) = filter.category_id {
            products_query = products_query.filter(category_id.eq(_category_id));
        }

        let mut product_ids_with_prices = product_store_prices
            .select(product_store_prices::product_id)
            .group_by(product_store_prices::product_id)
            .into_boxed();

        if let Some(min_price) = filter.min_price {
            product_ids_with_prices = product_ids_with_prices.filter(price.ge(min_price))
        }

        if let Some(max_price) = filter.max_price {
            product_ids_with_prices = product_ids_with_prices.filter(price.le(max_price))
        }

        products_query = products_query.filter(products::id.eq_any(product_ids_with_prices));

        let filtered_products = products_query.load::<Product>(conn)?;

        let prices_query = ProductStorePrice::belonging_to(&filtered_products)
            .select(ProductStorePrice::as_select())
            .into_boxed();

        let filtered_prices = prices_query.load::<ProductStorePrice>(conn)?;

        Ok(filtered_prices
            .grouped_by(&filtered_products)
            .into_iter()
            .zip(filtered_products)
            .map(|(prices, product)| Self::map_product_and_prices(conn, prices, product))
            .collect::<Vec<ProductDTO>>())
    }

    pub fn get_product(conn: &mut PgConnection, _product_id: i32) -> QueryResult<ProductDTO> {
        let product = products
            .filter(products::id.eq(_product_id))
            .first::<Product>(conn)?;

        let prices = ProductStorePrice::belonging_to(&product)
            .select(ProductStorePrice::as_select())
            .load::<ProductStorePrice>(conn)?;

        Ok(Self::map_product_and_prices(conn, prices, product))
    }

    pub fn find_product_store_by_id(
        conn: &mut PgConnection,
        _id: i32,
    ) -> QueryResult<ProductStore> {
        product_stores
            .filter(product_stores::id.eq(_id))
            .get_result::<ProductStore>(conn)
    }
}
