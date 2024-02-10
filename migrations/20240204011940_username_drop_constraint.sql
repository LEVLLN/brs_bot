-- Add migration script here
ALTER TABLE members DROP CONSTRAINT IF EXISTS members_username_key;
