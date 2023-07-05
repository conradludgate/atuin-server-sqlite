create table history (
    id integer primary key autoincrement,
    client_id text not null unique,
    user_id integer not null,
    hostname text not null,
    timestamp text not null,
    data text not null,
    created_at text not null default current_timestamp,
    deleted_at text
);

create unique index history_deleted_index on history(client_id, user_id, deleted_at);
