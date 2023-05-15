alter table product_store_prices
    alter column created_date type date using created_date::date;

