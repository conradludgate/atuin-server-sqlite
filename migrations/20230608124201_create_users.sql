create table users (
    id integer primary key autoincrement,
    username text not null unique,
    email text not null unique,
    password text not null unique,
    created_at text not null default (datetime('now','localtime'))
);

create unique index email_unique_idx on users (lower(email));
create unique index username_unique_idx on users (lower(username));
