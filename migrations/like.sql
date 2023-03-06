create table like (
    id          uuid primary key,
    message_id  uuid,
    created_at  timestamp       not null        default current_timestamp
);