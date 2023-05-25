alter table public.user_product_history
    add constraint user_product_history_users_id_fk
        foreign key (user_id) references public.users;
