alter table product_stores
    add price_id serial;

alter table product_stores
    add constraint product_stores_product_store_prices_id_fk
        foreign key (price_id) references product_store_prices;
