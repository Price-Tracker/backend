alter table user_product_history
    add id serial not null;

alter table user_product_history
    drop constraint user_product_history_pk;

alter table user_product_history
    add constraint user_product_history_pk
        primary key (id);

