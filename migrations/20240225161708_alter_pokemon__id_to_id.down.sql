-- Add down migration script here

ALTER TABLE pokemon
    RENAME id to _id;
ALTER TABLE pokemon
    ADD COLUMN id INT NULL;

ALTER TABLE language
    ADD COLUMN _id uuid;
UPDATE language
SET _id = gen_random_uuid()
WHERE _id IS NULL;
ALTER TABLE language
    ALTER COLUMN _id SET NOT NULL;