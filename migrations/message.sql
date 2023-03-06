create table message (
    id                  uuid primary key,
    author_id           uuid                not null,
    created_at          timestamp           not null        default current_timestamp,
    message             text                not null,
    message_parent_id   uuid
);