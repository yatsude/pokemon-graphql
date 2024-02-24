-- Add down migration script here

DROP TABLE language;

ALTER TABLE pokemon
    DROP CONSTRAINT IF EXISTS pokemon_order_key;