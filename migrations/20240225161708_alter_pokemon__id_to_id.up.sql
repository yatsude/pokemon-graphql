-- Add up migration script here

ALTER TABLE pokemon
    DROP COLUMN id;
ALTER TABLE pokemon
    RENAME _id TO id;

ALTER TABLE language
    DROP COLUMN _id;