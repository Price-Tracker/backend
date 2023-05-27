alter table user_shopping_carts
    add id serial;

alter table user_shopping_carts
    drop constraint user_shopping_carts_pk;

alter table user_shopping_carts
    add constraint user_shopping_carts_pk
        primary key (id);

