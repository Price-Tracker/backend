alter table delivered_notifications
    alter column created_date type timestamp using created_date::timestamp;

alter table product_stores
    alter column created_date type timestamp using created_date::timestamp;

alter table product_stores
    alter column updated_date type timestamp using updated_date::timestamp;

alter table products
    alter column created_date type timestamp using created_date::timestamp;

alter table products
    alter column updated_date type timestamp using updated_date::timestamp;

alter table stores
    alter column created_date type timestamp using created_date::timestamp;

alter table user_access
    alter column access_date type timestamp using access_date::timestamp;

alter table user_shopping_carts
    alter column created_date type timestamp using created_date::timestamp;

alter table user_subscribed_products
    alter column created_date type timestamp using created_date::timestamp;

alter table users
    alter column created_date type timestamp using created_date::timestamp;

alter table users
    alter column updated_date type timestamp using updated_date::timestamp;

