-- Add migration script here
UPDATE chats AS ch
SET substring_answer_chance = answer_packs.answer_chance
FROM answer_packs, answer_packs_to_chats
WHERE ch.id = answer_packs_to_chats.chat_id AND answer_packs.id = answer_packs_to_chats.pack_id AND answer_packs.name = 'based';