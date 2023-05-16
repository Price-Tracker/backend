alter table user_product_history
    add product_id serial not null;

alter table user_product_history
    add created_date timestamp default now() not null;

alter table user_product_history
    add constraint user_product_history_products_id_fk
        foreign key (product_id) references products;
