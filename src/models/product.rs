use crate::models::store::Store;
use crate::models::user::{UserSubscribedProduct, UserSubscribedProductDTO};
use crate::schema::product_store_prices::{self, dsl::*};
use crate::schema::product_stores::{self, dsl::*};
use crate::schema::products::{self, dsl::*};
use crate::schema::user_subscribed_products::{self, dsl::*};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{insert_into, update};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashSet;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, Queryable, Identifiable, Selectable, Serialize, ToSchema)]
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

#[derive(Queryable, Identifiable, Associations, Selectable, Serialize, ToSchema)]
#[diesel(belongs_to(Store))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = product_stores)]
pub struct ProductStore {
    pub id: i32,
    pub store_id: i32,
    pub product_id: i32,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Selectable, Serialize)]
#[diesel(belongs_to(ProductStore))]
#[diesel(table_name = product_store_prices)]
pub struct ProductStorePrice {
    pub price: f32,
    pub created_date: NaiveDateTime,
    pub id: i32,
    pub product_store_id: i32,
}

#[derive(Deserialize, IntoParams)]
pub struct ProductFilter {
    query: Option<String>,
    category_id: Option<i32>,
    min_price: Option<f32>,
    max_price: Option<f32>,
}

#[derive(Serialize, ToSchema)]
pub struct ProductStorePriceDTO {
    pub store_id: i32,
    pub store_name: String,
    pub product_store_id: i32,
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
        _product_store_id: i32,
    ) -> Option<ProductStorePrice> {
        product_store_prices
            .filter(product_store_id.eq(_product_store_id))
            .order(product_store_prices::created_date.desc())
            .first::<ProductStorePrice>(conn)
            .optional()
            .expect("Error loading latest price")
    }

    fn map_product_store_and_prices(
        conn: &mut PgConnection,
        prices: Vec<ProductStorePrice>,
        product_store: ProductStore,
    ) -> ProductDTO {
        let _product = products
            .select(Product::as_select())
            .filter(products::id.eq(product_store.product_id))
            .first(conn)
            .unwrap();
        let unique_store_ids: HashSet<i32> = prices.iter().map(|p| p.product_store_id).collect();
        let latest_prices: Vec<ProductStorePriceDTO> = unique_store_ids
            .iter()
            .filter_map(|_product_store_id| Self::find_latest_price(conn, *_product_store_id))
            .collect::<Vec<ProductStorePrice>>()
            .into_iter() // conn cannot be accessed simultaneously
            .map(|_price| {
                let _store_id =
                    Store::get_store_id_by_product_store_id(conn, _price.product_store_id).unwrap();
                ProductStorePriceDTO {
                    store_id: _store_id,
                    store_name: Store::get_store_name_by_id(conn, _store_id).unwrap(),
                    product_store_id: _price.product_store_id,
                    price: _price.price,
                }
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
            product: _product,
            prices: latest_prices,
            min_price,
            max_price,
        }
    }

    pub fn get_products_by_filter(
        conn: &mut PgConnection,
        filter: ProductFilter,
    ) -> QueryResult<Vec<ProductDTO>> {
        let mut products_query = products::table
            .select(Product::as_select())
            .inner_join(product_stores)
            .into_boxed();

        if let Some(query) = filter.query {
            let pattern = format!("%{}%", query);
            products_query = products_query.filter(name.ilike(pattern));
        }

        if let Some(_category_id) = filter.category_id {
            products_query = products_query.filter(category_id.eq(_category_id));
        }

        let mut product_store_ids_with_prices = product_store_prices
            .select(product_store_id)
            .group_by(product_store_id)
            .into_boxed();

        if let Some(min_price) = filter.min_price {
            product_store_ids_with_prices =
                product_store_ids_with_prices.filter(price.ge(min_price))
        }

        if let Some(max_price) = filter.max_price {
            product_store_ids_with_prices =
                product_store_ids_with_prices.filter(price.le(max_price))
        }

        products_query =
            products_query.filter(product_stores::id.eq_any(product_store_ids_with_prices));

        let filtered_products = products_query.load::<Product>(conn)?;

        let filtered_product_stores = ProductStore::belonging_to(&filtered_products)
            .select(ProductStore::as_select())
            .load::<ProductStore>(conn)?;

        let prices_query = ProductStorePrice::belonging_to(&filtered_product_stores)
            .select(ProductStorePrice::as_select())
            .into_boxed();

        let filtered_prices = prices_query.load::<ProductStorePrice>(conn)?;

        Ok(filtered_prices
            .grouped_by(&filtered_product_stores)
            .into_iter()
            .zip(filtered_product_stores)
            .map(|(prices, product_store)| {
                Self::map_product_store_and_prices(conn, prices, product_store)
            })
            .collect::<Vec<ProductDTO>>())
    }

    pub fn get_product(conn: &mut PgConnection, _product_id: i32) -> QueryResult<ProductDTO> {
        let product_store = product_stores
            .filter(product_stores::product_id.eq(_product_id))
            .first::<ProductStore>(conn)?;

        let prices = ProductStorePrice::belonging_to(&product_store)
            .select(ProductStorePrice::as_select())
            .load::<ProductStorePrice>(conn)?;

        Ok(Self::map_product_store_and_prices(
            conn,
            prices,
            product_store,
        ))
    }

    pub fn get_product_subscription(
        conn: &mut PgConnection,
        _user_id: i32,
        _product_id: i32,
    ) -> UserSubscribedProductDTO {
        if let Ok(subscription) = user_subscribed_products
            .filter(user_id.eq(_user_id))
            .filter(user_subscribed_products::product_id.eq(_product_id))
            .first::<UserSubscribedProduct>(conn)
        {
            UserSubscribedProductDTO {
                product_id: subscription.product_id,
                subscribed: true,
            }
        } else {
            UserSubscribedProductDTO {
                product_id: _product_id,
                subscribed: false,
            }
        }
    }

    pub fn subscribe_to_product(
        conn: &mut PgConnection,
        _user_id: i32,
        _product_id: i32,
    ) -> QueryResult<usize> {
        // check if user is already subscribed
        if let Ok(_) = user_subscribed_products
            .filter(user_id.eq(_user_id))
            .filter(user_subscribed_products::product_id.eq(_product_id))
            .first::<UserSubscribedProduct>(conn)
        {
            return Ok(0);
        }

        insert_into(user_subscribed_products::table)
            .values((
                user_id.eq(_user_id),
                user_subscribed_products::product_id.eq(_product_id),
            ))
            .execute(conn)
    }

    pub fn unsubscribe_from_product(
        conn: &mut PgConnection,
        _user_id: i32,
        _product_id: i32,
    ) -> QueryResult<usize> {
        // check if user is already unsubscribed
        if let Ok(_) = user_subscribed_products
            .filter(user_id.eq(_user_id))
            .filter(user_subscribed_products::product_id.eq(_product_id))
            .filter(subscribed.eq(false))
            .first::<UserSubscribedProduct>(conn)
        {
            return Ok(0);
        }

        update(user_subscribed_products::table)
            .filter(user_id.eq(_user_id))
            .filter(user_subscribed_products::product_id.eq(_product_id))
            .set(subscribed.eq(false))
            .execute(conn)
    }

    pub fn find_product_by_id(conn: &mut PgConnection, _id: i32) -> QueryResult<Product> {
        products
            .select(Product::as_select())
            .filter(products::id.eq(_id))
            .first(conn)
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
