create sequence user_id_seq
    as integer;

create sequence table_name_id_seq
    as integer;

create sequence table_name_company_id_seq
    as integer;

create table users
(
    id           serial             not null,
    nickname     varchar(16),
    login        varchar(16)        not null,
    email        varchar(32)        not null,
    password     varchar            not null,
    created_date date default now() not null,
    updated_date date default now() not null,
    constraint user_pk
        primary key (id)
);

alter sequence user_id_seq owned by users.id;

create table user_tokens
(
    id      serial,
    user_id serial,
    constraint user_tokens_pk
        primary key (id),
    constraint user_tokens_user_id_fk
        foreign key (user_id) references users
);

create table user_notification_settings
(
    user_id           serial,
    email_enabled     boolean default true not null,
    push_enabled      boolean default true not null,
    frequency_in_days integer default 3    not null,
    constraint user_notification_settings_pk
        primary key (user_id),
    constraint user_notification_settings_user_id_fk
        foreign key (user_id) references users
);

create table user_access
(
    id                  serial,
    user_id             serial,
    access_date         date default now() not null,
    is_successful_login boolean            not null,
    constraint user_access_pk
        primary key (id),
    constraint user_access_user_id_fk
        foreign key (user_id) references users
);

create table roles
(
    id          serial,
    name        varchar(15) not null,
    description varchar(60),
    constraint roles_pk
        primary key (id)
);

create table user_roles
(
    user_id serial,
    role_id serial,
    constraint user_roles_pk
        primary key (user_id),
    constraint user_roles_user_id_fk
        foreign key (user_id) references users,
    constraint user_roles_roles_id_fk
        foreign key (role_id) references roles
);

create table countries
(
    id   serial,
    name varchar(30) not null,
    constraint countries_pk
        primary key (id)
);

create table regions
(
    id         serial,
    country_id serial,
    name       varchar(30) not null,
    constraint regions_pk
        primary key (id),
    constraint regions_countries_id_fk
        foreign key (country_id) references countries
);

create table cities
(
    id        serial,
    region_id serial,
    name      varchar(30) not null,
    constraint cities_pk
        primary key (id),
    constraint cities_regions_id_fk
        foreign key (region_id) references regions
);

create table user_settings
(
    user_id      serial,
    main_country integer not null,
    main_city    integer,
    constraint user_settings_pk
        primary key (user_id),
    constraint user_settings_user_id_fk
        foreign key (user_id) references users,
    constraint user_settings_countries_id_fk
        foreign key (main_country) references countries,
    constraint user_settings_cities_id_fk
        foreign key (main_city) references cities
);

create table companies
(
    id   serial,
    name varchar(30) not null,
    constraint companies_pk
        primary key (id)
);

create table retail_chains
(
    id         serial      not null,
    company_id serial      not null,
    name       varchar(40) not null,
    website    varchar(40) not null,
    constraint table_name_pk
        primary key (id),
    constraint table_name_companies_id_fk
        foreign key (company_id) references companies
);

alter sequence table_name_id_seq owned by retail_chains.id;

alter sequence table_name_company_id_seq owned by retail_chains.company_id;

create table stores
(
    id              serial,
    retail_chain_id serial,
    country_id      serial,
    region_id       serial,
    city_id         serial,
    name            varchar(30)        not null,
    location        varchar(40)        not null,
    latitude        real,
    longitude       real,
    created_date    date default now() not null,
    constraint stores_pk
        primary key (id),
    constraint stores_cities_id_fk
        foreign key (city_id) references cities,
    constraint stores_countries_id_fk
        foreign key (country_id) references countries,
    constraint stores_regions_id_fk
        foreign key (region_id) references regions,
    constraint stores_retail_chains_id_fk
        foreign key (retail_chain_id) references retail_chains
);

create table categories
(
    id   serial,
    name varchar(30) not null,
    constraint categories_pk
        primary key (id)
);

create table products
(
    id           serial,
    category_id  serial,
    name         varchar(80)        not null,
    description  varchar,
    picture_url  varchar,
    created_date date default now() not null,
    updated_date date default now() not null,
    constraint products_pk
        primary key (id),
    constraint products_categories_id_fk
        foreign key (category_id) references categories
);

create table product_store_prices
(
    store_id     serial,
    product_id   serial,
    price        real               not null,
    created_date date default now() not null,
    id           serial,
    constraint product_store_prices_pk
        primary key (id),
    constraint product_store_prices_stores_id_fk
        foreign key (store_id) references stores,
    constraint product_store_prices_products_id_fk
        foreign key (product_id) references products
);

create table product_stores
(
    id           serial,
    store_id     serial,
    price_id     serial,
    product_id   serial,
    created_date date default now() not null,
    updated_date date default now() not null,
    constraint product_stores_pk
        primary key (id),
    constraint product_stores_stores_id_fk
        foreign key (store_id) references stores,
    constraint product_stores_product_store_prices_id_fk
        foreign key (price_id) references product_store_prices,
    constraint product_stores_products_id_fk
        foreign key (product_id) references products
);

create table user_shopping_carts
(
    user_id          serial,
    product_store_id serial,
    quantity         integer default 1     not null,
    created_date     date    default now() not null,
    constraint user_shopping_carts_pk
        primary key (user_id),
    constraint user_shopping_carts_product_stores_id_fk
        foreign key (product_store_id) references product_stores,
    constraint user_shopping_carts_user_id_fk
        foreign key (user_id) references users
);

create table user_product_history
(
    user_id serial,
    constraint user_product_history_pk
        primary key (user_id)
);

create table user_subscribed_products
(
    id                     serial,
    user_id                serial,
    product_id             serial,
    previous_minimal_price real                  not null,
    subscribed             boolean default true  not null,
    created_date           date    default now() not null,
    constraint user_subscribed_products_pk
        primary key (id),
    constraint user_subscribed_products_users_id_fk
        foreign key (user_id) references users,
    constraint user_subscribed_products___fk
        foreign key (product_id) references products
);

create table delivered_notifications
(
    id           serial,
    subscribe_id serial,
    type         varchar(20)        not null,
    delivered    boolean            not null,
    created_date date default now() not null,
    constraint delivered_notifications_pk
        primary key (id),
    constraint delivered_notifications_user_subscribed_products_id_fk
        foreign key (subscribe_id) references user_subscribed_products
);

create table user_product_review
(
    id          serial,
    product_id  serial,
    user_id     serial,
    store_id    serial,
    review_text varchar(120),
    score       integer               not null,
    checked     boolean default false not null,
    published   boolean default false not null,
    constraint user_product_review_pk
        primary key (id),
    constraint user_product_review_products_id_fk
        foreign key (product_id) references products,
    constraint user_product_review_users_id_fk
        foreign key (user_id) references users,
    constraint user_product_review_stores_id_fk
        foreign key (store_id) references stores
);

