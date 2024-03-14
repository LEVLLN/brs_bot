INSERT INTO public.answer_entities (is_active, created_at, updated_at, key, value, reaction_type, content_type,
                                    description, file_unique_id, chat_id)
VALUES (true, now(), now(), 'trigger_key', 'trigger_text_value',
        'TRIGGER', 'TEXT', null, null, 1);
