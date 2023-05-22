alter table delivered_notifications
    alter column created_date type date using created_date::date;

alter table product_stores
    alter column created_date type date using created_date::date;

alter table product_stores
    alter column updated_date type date using updated_date::date;

alter table products
    alter column created_date type date using created_date::date;

alter table products
    alter column updated_date type date using updated_date::date;

alter table stores
    alter column created_date type date using created_date::date;

alter table user_access
    alter column access_date type date using access_date::date;

alter table user_shopping_carts
    alter column created_date type date using created_date::date;

alter table user_subscribed_products
    alter column created_date type date using created_date::date;

alter table users
    alter column created_date type date using created_date::date;

alter table users
    alter column updated_date type date using updated_date::date;

