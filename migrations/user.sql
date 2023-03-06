create table user (
    id            uuid primary key,
    username      text                  unique not null,
    email         text                  unique not null,
    bio           text                  not null default '',
    image         text,
    password_hash text                  not null,
    created_at    timestamp             not null default current_timestamp,
    updated_at    timestamp
);