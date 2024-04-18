-- Your SQL goes here
create table wa_users
(
    id          serial primary key,
    name        varchar(100) not null,
    email       varchar(100) not null,
    pwd_hash    varchar(200),
    remark      varchar(500),
    create_time timestamp(3) not null default current_timestamp,
    update_time timestamp(3) not null default current_timestamp
);