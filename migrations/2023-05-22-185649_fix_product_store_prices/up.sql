alter table product_store_prices
    add product_store_id serial;

alter table product_store_prices
    drop constraint product_store_prices_stores_id_fk;

alter table product_store_prices
    drop column store_id;

alter table product_store_prices
    drop constraint product_store_prices_products_id_fk;

alter table product_store_prices
    drop column product_id;

alter table product_store_prices
    add constraint product_store_prices_product_stores_id_fk
        foreign key (product_store_id) references product_stores;

