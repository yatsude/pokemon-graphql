-- Add down migration script here

ALTER TABLE pokemon
    ALTER COLUMN id SET NOT NULL;