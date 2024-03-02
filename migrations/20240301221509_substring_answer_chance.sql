-- Add migration script here
ALTER TABLE chats ADD COLUMN substring_answer_chance int2 NOT NULL DEFAULT 15;