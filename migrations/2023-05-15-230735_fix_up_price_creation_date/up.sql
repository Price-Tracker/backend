alter table product_store_prices
    alter column created_date type timestamp using created_date::timestamp;
