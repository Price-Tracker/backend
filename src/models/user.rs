use crate::config::app::Config;
use crate::models::product::{Product, ProductStore};
use crate::models::user_tokens::{UserRefreshTokenDTO, UserToken, UserTokensDTO};
use crate::schema::user_product_history::user_id;
use crate::schema::user_product_history::{self, dsl::*};
use crate::schema::user_shopping_carts;
use crate::schema::user_subscribed_products;
use crate::schema::users::{self, dsl::*};
use actix_web::web::Data;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::NaiveDateTime;
use diesel::insert_into;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PasswordRequirements {
    pub min_length: u32,
    pub max_length: u32,
    pub min_uppercase: u32,
    pub min_lowercase: u32,
    pub min_digits: u32,
    pub min_special: u32,
}

impl PasswordRequirements {
    pub fn default() -> PasswordRequirements {
        PasswordRequirements {
            min_length: 8,
            max_length: 32,
            min_uppercase: 1,
            min_lowercase: 1,
            min_digits: 1,
            min_special: 1,
        }
    }

    pub fn validate(&self, _password: &str) -> Result<(), String> {
        let mut uppercase = 0;
        let mut lowercase = 0;
        let mut digits = 0;
        let mut special = 0;

        for c in _password.chars() {
            if c.is_ascii_uppercase() {
                uppercase += 1;
            } else if c.is_ascii_lowercase() {
                lowercase += 1;
            } else if c.is_ascii_digit() {
                digits += 1;
            } else {
                special += 1;
            }
        }

        if _password.len() < self.min_length as usize {
            return Err(format!(
                "Password must be at least {} characters long",
                self.min_length
            ));
        }

        if _password.len() > self.max_length as usize {
            return Err(format!(
                "Password must be at most {} characters long",
                self.max_length
            ));
        }

        if uppercase < self.min_uppercase {
            return Err(format!(
                "Password must have at least {} uppercase characters",
                self.min_uppercase
            ));
        }

        if lowercase < self.min_lowercase {
            return Err(format!(
                "Password must have at least {} lowercase characters",
                self.min_lowercase
            ));
        }

        if digits < self.min_digits {
            return Err(format!(
                "Password must have at least {} digits",
                self.min_digits
            ));
        }

        if special < self.min_special {
            return Err(format!(
                "Password must have at least {} special characters",
                self.min_special
            ));
        }

        Ok(())
    }
}

#[derive(Identifiable, Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub nickname: Option<String>,
    pub login: String,
    pub email: String,
    pub password: String,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(Queryable, Associations, Selectable, Insertable, Serialize)]
#[diesel(belongs_to(Product))]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_product_history)]
pub struct UserProductHistory {
    pub user_id: i32,
    pub product_id: i32,
    pub created_date: NaiveDateTime,
}

#[derive(Queryable, Associations, Selectable, Insertable, Identifiable, Serialize)]
#[diesel(belongs_to(ProductStore))]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_shopping_carts)]
pub struct UserShoppingCart {
    pub user_id: i32,
    pub product_store_id: i32,
    pub quantity: i32,
    pub created_date: NaiveDateTime,
    pub id: i32,
}

#[derive(Queryable, Associations, Selectable, Insertable, Identifiable, Serialize)]
#[diesel(belongs_to(Product))]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_subscribed_products)]
pub struct UserSubscribedProduct {
    pub id: i32,
    pub user_id: i32,
    pub product_id: i32,
    pub previous_minimal_price: Option<f32>,
    pub subscribed: bool,
    pub created_date: NaiveDateTime,
}

#[derive(Queryable, Associations, Selectable, Insertable, Serialize)]
#[diesel(belongs_to(ProductStore))]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_shopping_carts)]
pub struct UserShoppingCartInsertable {
    pub user_id: i32,
    pub product_store_id: i32,
    pub quantity: i32,
}

#[derive(Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub login: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = user_shopping_carts)]
pub struct UserShoppingCartDTO {
    pub product_store_id: i32,
    pub quantity: i32,
}

#[derive(Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = user_subscribed_products)]
pub struct UserSubscribedProductDTO {
    pub product_id: i32,
    pub subscribed: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginDTO {
    pub login_or_email: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct HistoryDTO {
    product_id: i32,
}

#[derive(Serialize, ToSchema)]
pub struct HistoryWithProductDTO {
    product: Product,
    access_date: NaiveDateTime,
}

impl User {
    pub fn signup(conn: &mut PgConnection, user: UserDTO) -> Result<String, String> {
        if Self::find_user_by_login(conn, &user.login).is_err()
            && Self::find_user_by_email(conn, &user.email).is_err()
        {
            // Validate password requirements
            let password_requirements = Self::get_password_requirements();
            if let Err(e) = password_requirements.validate(&user.password) {
                return Err(e);
            }

            let salt = SaltString::generate(&mut OsRng);
            let hashed_password = Argon2::default()
                .hash_password(user.password.as_bytes(), &salt)
                .expect("Error while hashing password")
                .to_string();

            let user = UserDTO {
                password: hashed_password,
                ..user
            };

            insert_into(users).values(user).execute(conn).unwrap();

            Ok("Signup successfully".to_string())
        } else {
            Err(format!(
                "Login '{}' or Email '{}' is already registered",
                &user.login, &user.email
            ))
        }
    }

    pub fn login(
        conn: &mut PgConnection,
        login_cred: LoginDTO,
        config: Data<Config>,
    ) -> Result<UserTokensDTO, String> {
        if let Ok(fetched_user) = users
            .filter(login.eq(&login_cred.login_or_email))
            .or_filter(email.eq(&login_cred.login_or_email))
            .get_result::<User>(conn)
        {
            if let Ok(parsed_hash) = PasswordHash::new(&fetched_user.password) {
                if Argon2::default()
                    .verify_password(login_cred.password.as_bytes(), &parsed_hash)
                    .is_ok()
                {
                    let access_token =
                        UserToken::generate_access_token(fetched_user.clone(), config);
                    let refresh_token =
                        UserToken::generate_refresh_token(conn, fetched_user.clone());

                    let user_tokens = UserTokensDTO {
                        access_token,
                        refresh_token,
                    };

                    return Ok(user_tokens);
                }
            }
        }
        Err("Login, email or password is wrong!".to_string())
    }

    pub fn refresh_token(
        conn: &mut PgConnection,
        user_refresh_token: UserRefreshTokenDTO,
        config: Data<Config>,
    ) -> Result<UserTokensDTO, String> {
        if let Ok(user_token) =
            UserToken::find_refresh_token(conn, user_refresh_token.refresh_token.clone())
        {
            let user_tokens = UserToken::refresh_tokens(conn, user_token, config);

            Ok(user_tokens)
        } else {
            Err("Refresh token not found!".to_string())
        }
    }

    pub fn get_password_requirements() -> PasswordRequirements {
        PasswordRequirements::default()
    }

    pub fn add_to_history(
        conn: &mut PgConnection,
        _user_id: i32,
        history_dto: HistoryDTO,
    ) -> QueryResult<usize> {
        // TODO: rework when new fields will be added
        insert_into(user_product_history)
            .values((
                user_id.eq(_user_id),
                user_product_history::product_id.eq(history_dto.product_id),
            ))
            .execute(conn)
    }

    pub fn get_history(
        conn: &mut PgConnection,
        _user_id: i32,
    ) -> QueryResult<Vec<HistoryWithProductDTO>> {
        let mut fetched_history = user_product_history
            .select(UserProductHistory::as_select())
            .filter(user_id.eq(_user_id))
            .distinct_on(user_product_history::product_id)
            .limit(60) // TODO: make it configurable
            .order_by((
                user_product_history::product_id,
                user_product_history::created_date.desc(),
            ))
            .get_results(conn)?
            .into_iter()
            .map(|history| HistoryWithProductDTO {
                product: Product::find_product_by_id(conn, history.product_id).unwrap(),
                access_date: history.created_date,
            })
            .collect::<Vec<HistoryWithProductDTO>>();

        fetched_history.sort_by(|a, b| b.access_date.cmp(&a.access_date));

        Ok(fetched_history)
    }

    pub fn add_to_cart(
        conn: &mut PgConnection,
        _user_id: i32,
        cart_dto: UserShoppingCartDTO,
    ) -> QueryResult<usize> {
        // Ensure that this product_store exists
        let product_store = Product::find_product_store_by_id(conn, cart_dto.product_store_id)?;

        let insertable_cart_item = UserShoppingCartInsertable {
            user_id: _user_id,
            product_store_id: product_store.id,
            quantity: cart_dto.quantity,
        };

        // Check if this product_store is already in cart for this user
        if let Ok(cart_item) = user_shopping_carts::dsl::user_shopping_carts
            .filter(user_shopping_carts::user_id.eq(_user_id))
            .filter(user_shopping_carts::product_store_id.eq(product_store.id))
            .first::<UserShoppingCart>(conn)
        {
            // If it is, then just update quantity
            return diesel::update(&cart_item)
                .set(
                    user_shopping_carts::quantity
                        .eq(cart_dto.quantity + insertable_cart_item.quantity),
                )
                .execute(conn);
        }

        insert_into(user_shopping_carts::dsl::user_shopping_carts)
            .values(insertable_cart_item)
            .execute(conn)
    }

    pub fn get_cart(
        conn: &mut PgConnection,
        _user_id: i32,
    ) -> QueryResult<Vec<UserShoppingCartDTO>> {
        Ok(user_shopping_carts::dsl::user_shopping_carts
            .filter(user_shopping_carts::user_id.eq(_user_id))
            .get_results::<UserShoppingCart>(conn)?
            .into_iter()
            .map(|cart_item| UserShoppingCartDTO {
                product_store_id: cart_item.product_store_id,
                quantity: cart_item.quantity,
            })
            .collect::<Vec<UserShoppingCartDTO>>())
    }

    pub fn get_subscriptions(
        conn: &mut PgConnection,
        _user_id: i32,
    ) -> QueryResult<Vec<UserSubscribedProductDTO>> {
        Ok(user_subscribed_products::dsl::user_subscribed_products
            .filter(user_subscribed_products::user_id.eq(_user_id))
            .filter(user_subscribed_products::subscribed.eq(true))
            .get_results::<UserSubscribedProduct>(conn)?
            .into_iter()
            .map(|subscribed_product| UserSubscribedProductDTO {
                product_id: subscribed_product.product_id,
                subscribed: subscribed_product.subscribed,
            })
            .collect::<Vec<UserSubscribedProductDTO>>())
    }

    pub fn find_user_by_id(conn: &mut PgConnection, _id: i32) -> QueryResult<User> {
        users.filter(users::id.eq(_id)).get_result::<User>(conn)
    }

    pub fn find_user_by_login(conn: &mut PgConnection, _login: &str) -> QueryResult<User> {
        users.filter(login.eq(_login)).get_result::<User>(conn)
    }

    pub fn find_user_by_email(conn: &mut PgConnection, _email: &str) -> QueryResult<User> {
        users.filter(email.eq(_email)).get_result::<User>(conn)
    }
}
