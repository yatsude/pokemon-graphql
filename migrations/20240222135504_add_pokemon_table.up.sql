-- Add up migration script here
-- Create table pokemon
CREATE TABLE pokemon
(
    _id             uuid    NOT NULL,
    id              INT     NOT NULL UNIQUE,
    PRIMARY KEY (_id),
    name            TEXT    NOT NULL,
    base_experience INT     NOT NULL,
    height          INT     NOT NULL,
    is_default      BOOLEAN NOT NULL,
    "order"         INT     NOT NULL
);