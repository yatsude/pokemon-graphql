-- Add up migration script here

ALTER TABLE pokemon
    ALTER COLUMN id DROP NOT NULL;