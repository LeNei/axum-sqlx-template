-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table users (
    id uuid not null primary key default uuid_generate_v4(),
    username varchar(255) not null unique,
    password varchar(255) not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz  not null default now()
);

select manage_updated_at('users');
