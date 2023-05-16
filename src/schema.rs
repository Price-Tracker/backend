// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    cities (id) {
        id -> Int4,
        region_id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    companies (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    countries (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    delivered_notifications (id) {
        id -> Int4,
        subscribe_id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        delivered -> Bool,
        created_date -> Date,
    }
}

diesel::table! {
    product_store_prices (id) {
        store_id -> Int4,
        product_id -> Int4,
        price -> Float4,
        created_date -> Timestamp,
        id -> Int4,
    }
}

diesel::table! {
    product_stores (id) {
        id -> Int4,
        store_id -> Int4,
        price_id -> Int4,
        product_id -> Int4,
        created_date -> Date,
        updated_date -> Date,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        category_id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        picture_url -> Nullable<Varchar>,
        created_date -> Date,
        updated_date -> Date,
    }
}

diesel::table! {
    regions (id) {
        id -> Int4,
        country_id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    retail_chains (id) {
        id -> Int4,
        company_id -> Int4,
        name -> Varchar,
        website -> Varchar,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    stores (id) {
        id -> Int4,
        retail_chain_id -> Int4,
        country_id -> Int4,
        region_id -> Int4,
        city_id -> Int4,
        name -> Varchar,
        location -> Varchar,
        latitude -> Nullable<Float4>,
        longitude -> Nullable<Float4>,
        created_date -> Date,
    }
}

diesel::table! {
    user_access (id) {
        id -> Int4,
        user_id -> Int4,
        access_date -> Date,
        is_successful_login -> Bool,
    }
}

diesel::table! {
    user_notification_settings (user_id) {
        user_id -> Int4,
        email_enabled -> Bool,
        push_enabled -> Bool,
        frequency_in_days -> Int4,
    }
}

diesel::table! {
    user_product_history (user_id) {
        user_id -> Int4,
        product_id -> Int4,
        created_date -> Timestamp,
    }
}

diesel::table! {
    user_product_review (id) {
        id -> Int4,
        product_id -> Int4,
        user_id -> Int4,
        store_id -> Int4,
        review_text -> Nullable<Varchar>,
        score -> Int4,
        checked -> Bool,
        published -> Bool,
    }
}

diesel::table! {
    user_roles (user_id) {
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::table! {
    user_settings (user_id) {
        user_id -> Int4,
        main_country -> Int4,
        main_city -> Nullable<Int4>,
    }
}

diesel::table! {
    user_shopping_carts (user_id) {
        user_id -> Int4,
        product_store_id -> Int4,
        quantity -> Int4,
        created_date -> Date,
    }
}

diesel::table! {
    user_subscribed_products (id) {
        id -> Int4,
        user_id -> Int4,
        product_id -> Int4,
        previous_minimal_price -> Float4,
        subscribed -> Bool,
        created_date -> Date,
    }
}

diesel::table! {
    user_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        refresh_token -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        nickname -> Nullable<Varchar>,
        login -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_date -> Date,
        updated_date -> Date,
    }
}

diesel::joinable!(cities -> regions (region_id));
diesel::joinable!(delivered_notifications -> user_subscribed_products (subscribe_id));
diesel::joinable!(product_store_prices -> products (product_id));
diesel::joinable!(product_store_prices -> stores (store_id));
diesel::joinable!(product_stores -> product_store_prices (price_id));
diesel::joinable!(product_stores -> products (product_id));
diesel::joinable!(product_stores -> stores (store_id));
diesel::joinable!(products -> categories (category_id));
diesel::joinable!(regions -> countries (country_id));
diesel::joinable!(retail_chains -> companies (company_id));
diesel::joinable!(stores -> cities (city_id));
diesel::joinable!(stores -> countries (country_id));
diesel::joinable!(stores -> regions (region_id));
diesel::joinable!(stores -> retail_chains (retail_chain_id));
diesel::joinable!(user_access -> users (user_id));
diesel::joinable!(user_notification_settings -> users (user_id));
diesel::joinable!(user_product_history -> products (product_id));
diesel::joinable!(user_product_review -> products (product_id));
diesel::joinable!(user_product_review -> stores (store_id));
diesel::joinable!(user_product_review -> users (user_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(user_settings -> cities (main_city));
diesel::joinable!(user_settings -> countries (main_country));
diesel::joinable!(user_settings -> users (user_id));
diesel::joinable!(user_shopping_carts -> product_stores (product_store_id));
diesel::joinable!(user_shopping_carts -> users (user_id));
diesel::joinable!(user_subscribed_products -> products (product_id));
diesel::joinable!(user_subscribed_products -> users (user_id));
diesel::joinable!(user_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    cities,
    companies,
    countries,
    delivered_notifications,
    product_store_prices,
    product_stores,
    products,
    regions,
    retail_chains,
    roles,
    stores,
    user_access,
    user_notification_settings,
    user_product_history,
    user_product_review,
    user_roles,
    user_settings,
    user_shopping_carts,
    user_subscribed_products,
    user_tokens,
    users,
);
