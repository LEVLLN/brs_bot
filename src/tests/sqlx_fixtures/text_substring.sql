INSERT INTO public.answer_entities (is_active, created_at, updated_at, key, value, reaction_type, content_type,
                                    description, file_unique_id, chat_id)
VALUES (true, now(), now(), 'substring_key', 'substring_text_value',
        'SUBSTRING', 'TEXT', null, null, 1);
