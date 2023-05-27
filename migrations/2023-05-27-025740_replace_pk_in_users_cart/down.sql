alter table user_shopping_carts
    drop constraint user_shopping_carts_pk;

alter table user_shopping_carts
    drop column id;

alter table user_shopping_carts
    add constraint user_shopping_carts_pk
        primary key (user_id);

