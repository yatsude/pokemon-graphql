-- Add up migration script here

CREATE TABLE account
(
    id         uuid        NOT NULL,
    PRIMARY KEY (id),
    email      TEXT        NOT NULL UNIQUE,
    name       TEXT        NOT NULL,
    created_at timestamptz NOT NULL default now()
);