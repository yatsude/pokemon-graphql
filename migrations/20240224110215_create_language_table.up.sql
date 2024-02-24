-- Add up migration script here

-- Create table language
CREATE TABLE language
(
    _id      uuid    NOT NULL,
    PRIMARY KEY (_id),
    id       INT     NOT NULL UNIQUE,
    iso3166  TEXT    NOT NULL,
    iso639   TEXT    NOT NULL,
    name     TEXT    NOT NULL,
    official BOOLEAN NOT NULL,
    "order"  INT     NOT NULL UNIQUE
);

-- add unique to table order
ALTER TABLE pokemon
    ADD UNIQUE ("order");