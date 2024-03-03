-- Add migration script here
ALTER TABLE answer_entities ADD COLUMN chat_id INT NULL CONSTRAINT answer_packs_chat_fk_chat_id REFERENCES chats (id) ON DELETE CASCADE;