-- Add migration script here
UPDATE answer_entities AS ae
SET chat_id = answer_packs_to_chats.chat_id
FROM answer_packs_to_chats
WHERE ae.pack_id = answer_packs_to_chats.pack_id;
ALTER TABLE answer_entities ALTER COLUMN chat_id SET NOT NULL;
ALTER TABLE answer_entities DROP COLUMN pack_id;
DROP TABLE answer_packs_to_chats;
DROP TABLE answer_packs;