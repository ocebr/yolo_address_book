-- Add migration script here

create table friends_list (
    id serial PRIMARY KEY,
    id_user varchar not null,
    friend varchar not null,
    created_at timestamp not null default current_timestamp
);

-- pour deux on conflict create index -> create unique index idx_t_id_a on friends_list (id, friend);