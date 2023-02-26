-- Add migration script here
ALTER TABLE songs
ADD deleted_at INT DEFAULT NULL;
