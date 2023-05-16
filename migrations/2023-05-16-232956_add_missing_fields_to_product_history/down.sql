alter table user_product_history
    drop column created_date;

alter table user_product_history
    drop constraint user_product_history_products_id_fk;

alter table user_product_history
    drop column product_id;

