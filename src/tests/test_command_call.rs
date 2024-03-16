#[cfg(test)]
mod tests {
    use crate::common::db::{AnswerEntity, Chat as ChatDB, ChatId, DictionaryEntity, EntityReactionType, MemberId};
    use crate::common::error::ProcessError;
    use assert_json_diff::assert_json_include;
    use serde_json::json;
    use sqlx::PgPool;

    use crate::common::lexer::tokenize;
    use crate::common::message_service::{handle_processor, Processor};
    use crate::common::response::ResponseMessage;
    use crate::tests::helpers::fixtures::{
        db_existed_chat_member, default_origin_direct_text_message, replied_text_message,
        request_existed_chat_user, roll_callback_message,
    };

    async fn call_command_direct(pool: &PgPool, input_text: &str) -> ResponseMessage {
        let (user, chat) = request_existed_chat_user().await;
        let (member_db_id, chat_db_id, chat_to_member_db_id) = db_existed_chat_member(pool).await;
        handle_processor(
            &Processor::Command,
            &Some(tokenize(input_text)),
            &default_origin_direct_text_message(&user, &chat, input_text),
            pool,
            &member_db_id,
            &chat_db_id,
            &chat_to_member_db_id,
        )
        .await
        .unwrap()
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(path = "sqlx_fixtures", scripts("default_chat", "default_user"))
    )]
    async fn test_callback(pool: PgPool) {
        let (user, chat) = request_existed_chat_user().await;
        let (member_db_id, chat_db_id, chat_to_member_db_id) = db_existed_chat_member(&pool).await;
        let request_payload = roll_callback_message(&user, &chat, "хлеб кто динозавр");
        let tokens = &Some(tokenize("Some answer in previous command"));
        let result = handle_processor(
            &Processor::Callback,
            tokens,
            &request_payload,
            &pool,
            &member_db_id,
            &chat_db_id,
            &chat_to_member_db_id,
        )
        .await
        .unwrap();
        assert_json_include!(
            actual: json!(result),
            expected: json!({
                "reply_to_message_id": 2,
                "text": "FirstName LastName динозавр",
                "reply_markup": {"inline_keyboard": [[{"text": "Roll", "callback_data": ""}]]}
            })
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(path = "sqlx_fixtures", scripts("default_chat", "default_user"))
    )]
    async fn test_who(pool: PgPool) {
        for (input, output) in [
            ("хлеб кто собака", "FirstName LastName собака"),
            ("хлеб кто динозавр?", "FirstName LastName динозавр"),
            ("хлеб кто", "FirstName LastName"),
            ("хлеб у кого", "у него(неё): FirstName LastName"),
            (
                "хлеб у кого динозавр?",
                "динозавр у него(неё): FirstName LastName",
            ),
            (
                "хлеб про кого песня?",
                "песня про него(неё): FirstName LastName",
            ),
        ] {
            assert_json_include!(
                actual: json!(call_command_direct(&pool, input).await),
                expected: json!({"text": output})
            );
        }
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(path = "sqlx_fixtures", scripts("default_chat", "default_user"))
    )]
    async fn test_show_answer_chance(pool: PgPool) {
        for (input, output) in [
            ("хлеб процент", "12"),
            ("хлеб процент подстрок", "12"),
            ("хлеб процент бреда", "11"),
        ] {
            assert_json_include!(
                actual: json!(call_command_direct(&pool, input).await),
                expected: json!({"text": output})
            );
        }
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(path = "sqlx_fixtures", scripts("default_chat", "default_user"))
    )]
    async fn test_answer_chance_failure<'a>(pool: PgPool) {
        let (user, chat) = request_existed_chat_user().await;
        let (member_db_id, chat_db_id, chat_to_member_db_id) = db_existed_chat_member(&pool).await;
        for (input, user_id, chat_id, output) in [
            (
                "хлеб процент string",
                &member_db_id,
                &chat_db_id,
                "Указано неверное значение. Должно быть целое число от 0 до 100",
            ),
            (
                "хлеб процент 199",
                &member_db_id,
                &chat_db_id,
                "Указано неверное значение. Должно быть целое число от 0 до 100",
            ),
            (
                "хлеб процент подстрок -199",
                &member_db_id,
                &chat_db_id,
                "Указано неверное значение. Должно быть целое число от 0 до 100",
            ),
            (
                "хлеб процент триггер 100",
                &member_db_id,
                &chat_db_id,
                "Указано неверное значение. Должно быть целое число от 0 до 100",
            ),
            (
                "хлеб процент",
                &MemberId::new(100500),
                &ChatId::new(10550),
                "Не заполнен процент срабатывания подстрок",
            ),
            (
                "хлеб процент 10",
                &MemberId::new(100500),
                &ChatId::new(10550),
                "Произошла ошибка обновления процента срабатывания подстрок",
            ),
        ] {
            let request_payload = default_origin_direct_text_message(&user, &chat, input);
            let tokens = &Some(tokenize(input));
            if let ProcessError::Feedback { message: msg } = handle_processor(
                &Processor::Command,
                tokens,
                &request_payload,
                &pool,
                user_id,
                chat_id,
                &chat_to_member_db_id,
            )
            .await
            .unwrap_err()
            {
                assert_eq!(msg, output);
            } else {
                panic!("Assertion error: {}", output);
            }
        }
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(path = "sqlx_fixtures", scripts("default_chat", "default_user"))
    )]
    async fn test_set_morph_answer_chance(pool: PgPool) {
        call_command_direct(&pool, "хлеб процент бреда 10").await;
        let (_, chat_db_id, _) = db_existed_chat_member(&pool).await;
        assert_eq!(
            Some(10),
            ChatDB::morph_answer_chance(&pool, &chat_db_id).await
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(path = "sqlx_fixtures", scripts("default_chat", "default_user"))
    )]
    async fn test_set_substring_answer_chance(pool: PgPool) {
        call_command_direct(&pool, "хлеб процент 25").await;
        let (_, chat_db_id, _) = db_existed_chat_member(&pool).await;
        assert_eq!(
            Some(25),
            ChatDB::substring_answer_chance(&pool, &chat_db_id).await
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(path = "sqlx_fixtures", scripts("default_chat", "default_user"))
    )]
    async fn test_help(pool: PgPool) {
        for (input, output) in [
            ("хлеб хелп", "Привет. Я бот и меня зовут Хлебушек."),
            ("хлеб хелп команды", "цит:"),
            ("хлеб хелп механика", "Основные элементы"),
            ("хлеб хелп бред", r#"Команда: \"бред\""#),
        ] {
            assert!(json!(call_command_direct(&pool, input).await)
                .get("text")
                .unwrap()
                .to_string()
                .contains(output));
        }
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(path = "sqlx_fixtures", scripts("default_chat", "default_user"))
    )]
    async fn test_show_without_data(pool: PgPool) {
        let (user, chat) = request_existed_chat_user().await;
        let (member_db_id, chat_db_id, chat_to_member_db_id) = db_existed_chat_member(&pool).await;
        let request_payload =
            default_origin_direct_text_message(&user, &chat, "хлеб проверь несуществующий_ключ");
        let tokens = &Some(tokenize("хлеб проверь несуществующий_ключ"));
        if let ProcessError::Feedback { message: msg } = handle_processor(
            &Processor::Command,
            tokens,
            &request_payload,
            &pool,
            &member_db_id,
            &chat_db_id,
            &chat_to_member_db_id,
        )
        .await
        .unwrap_err()
        {
            assert_eq!(msg, "Ничего не было найдено");
        } else {
            panic!("Assertion error");
        }
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(
            path = "sqlx_fixtures",
            scripts("default_chat", "default_user", "text_substring")
        )
    )]
    async fn test_show_text_substring(pool: PgPool) {
        if let ResponseMessage::Text { text, .. } =
            call_command_direct(&pool, "хлеб проверь substring_key").await
        {
            assert_eq!(text, "substring_text_value");
        }
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(
            path = "sqlx_fixtures",
            scripts("default_chat", "default_user", "text_trigger")
        )
    )]
    async fn test_show_text_trigger(pool: PgPool) {
        if let ResponseMessage::Text { text, .. } =
            call_command_direct(&pool, "хлеб проверь триггер trigger_key").await
        {
            assert_eq!(text, "trigger_text_value");
        }
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(
            path = "sqlx_fixtures",
            scripts("default_chat", "default_user", "text_substring")
        )
    )]
    async fn test_show_keys_failure(pool: PgPool) {
        let (user, chat) = request_existed_chat_user().await;
        let (member_db_id, chat_db_id, chat_to_member_db_id) = db_existed_chat_member(&pool).await;
        let request_payload = default_origin_direct_text_message(&user, &chat, "хлеб покажи ключи");
        let tokens = &Some(tokenize("хлеб покажи ключи"));
        if let ProcessError::Feedback { message: msg } = handle_processor(
            &Processor::Command,
            tokens,
            &request_payload,
            &pool,
            &member_db_id,
            &chat_db_id,
            &chat_to_member_db_id,
        )
        .await
        .unwrap_err()
        {
            assert_eq!(msg, "Необходимо выбрать сообщение в ответ");
        } else {
            panic!("Assertion error");
        }
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(
            path = "sqlx_fixtures",
            scripts("default_chat", "default_user", "text_substring")
        )
    )]
    async fn test_show_keys_not_found(pool: PgPool) {
        let (user, chat) = request_existed_chat_user().await;
        let (member_db_id, chat_db_id, chat_to_member_db_id) = db_existed_chat_member(&pool).await;
        let request_payload =
            replied_text_message(&user, &chat, "хлеб покажи ключи", "wrong_value");
        let tokens = &Some(tokenize("хлеб покажи ключи"));
        if let ProcessError::Feedback { message: msg } = handle_processor(
            &Processor::Command,
            tokens,
            &request_payload,
            &pool,
            &member_db_id,
            &chat_db_id,
            &chat_to_member_db_id,
        )
        .await
        .unwrap_err()
        {
            assert_eq!(msg, "Ключей не найдено");
        } else {
            panic!("Assertion error");
        }
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(
            path = "sqlx_fixtures",
            scripts("default_chat", "default_user", "text_substring")
        )
    )]
    async fn test_show_keys(pool: PgPool) {
        let (user, chat) = request_existed_chat_user().await;
        let (member_db_id, chat_db_id, chat_to_member_db_id) = db_existed_chat_member(&pool).await;
        let request_payload =
            replied_text_message(&user, &chat, "хлеб покажи ключи", "substring_text_value");
        let tokens = &Some(tokenize("хлеб покажи ключи"));
        let result = handle_processor(
            &Processor::Command,
            tokens,
            &request_payload,
            &pool,
            &member_db_id,
            &chat_db_id,
            &chat_to_member_db_id,
        )
        .await
        .unwrap();
        assert_json_include!(
            actual: json!(result),
            expected: json!({
                "reply_to_message_id": 2,
                "text": "substring_key",
            })
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(
            path = "sqlx_fixtures",
            scripts("default_chat", "default_user", "text_substring")
        )
    )]
    async fn test_remember(pool: PgPool) {
        let (user, chat) = request_existed_chat_user().await;
        let (member_db_id, chat_db_id, chat_to_member_db_id) = db_existed_chat_member(&pool).await;
        let request_payload = replied_text_message(
            &user,
            &chat,
            "хлеб запомни булочка, фонарь-истребитель, аптека вертолет, substring_key",
            "substring_text_value",
        );
        let tokens = &Some(tokenize(
            "хлеб запомни булочка, фонарь-истребитель, аптека вертолет, substring_key",
        ));
        let result = handle_processor(
            &Processor::Command,
            tokens,
            &request_payload,
            &pool,
            &member_db_id,
            &chat_db_id,
            &chat_to_member_db_id,
        )
        .await
        .unwrap();
        assert_json_include!(
            actual: json!(result),
            expected: json!({
                "text": "Сделал"
            })
        );
        for _expected in [
            "булочка",
            "фонарь-истребитель",
            "аптека вертолет",
            "substring_key",
        ] {
            let existed_entities = AnswerEntity::find_values_by_keys(
                &pool,
                &chat_db_id,
                &[_expected.to_string()],
                &EntityReactionType::Substring,
            )
            .await;
            assert_eq!(existed_entities.len(), 1);
            assert_eq!(existed_entities[0].value, "substring_text_value")
        }
    }
    
    #[sqlx::test(
    migrations = "./migrations",
    fixtures(
    path = "sqlx_fixtures",
    scripts("default_chat", "default_user", "text_substring")
    )
    )]
    async fn test_delete(pool: PgPool) {
        let (user, chat) = request_existed_chat_user().await;
        let (member_db_id, chat_db_id, chat_to_member_db_id) = db_existed_chat_member(&pool).await;
        let request_payload = replied_text_message(
            &user,
            &chat,
            "хлеб удали подстроку",
            "substring_text_value",
        );
        let tokens = &Some(tokenize(
            "хлеб удали",
        ));
        assert!(!AnswerEntity::find_values_by_keys(
            &pool,
            &chat_db_id,
            &["substring_key".to_string()],
            &EntityReactionType::Substring,
        )
            .await.is_empty());
        let result = handle_processor(
            &Processor::Command,
            tokens,
            &request_payload,
            &pool,
            &member_db_id,
            &chat_db_id,
            &chat_to_member_db_id,
        )
            .await
            .unwrap();
        assert_json_include!(
            actual: json!(result),
            expected: json!({
                "text": "Был удален контент на ключах: substring_key"
            })
        );
        assert!(AnswerEntity::find_values_by_keys(
            &pool,
            &chat_db_id,
            &["substring_key".to_string()],
            &EntityReactionType::Substring,
        )
            .await.is_empty());
    }

    #[sqlx::test(
    migrations = "./migrations",
    fixtures(
    path = "sqlx_fixtures",
    scripts("default_chat", "default_user")
    )
    )]
    async fn test_add_to_dictionary(pool: PgPool) {
        let (user, chat) = request_existed_chat_user().await;
        let (member_db_id, chat_db_id, chat_to_member_db_id) = db_existed_chat_member(&pool).await;
        let request_payload = default_origin_direct_text_message(&user, &chat, "хлеб добавь бред булочка, ёлочка, батончик");
        let tokens = &Some(tokenize(
            "хлеб добавь бред Булочка, ёлочка, батончик",
        ));
        let result = handle_processor(
            &Processor::Command,
            tokens,
            &request_payload,
            &pool,
            &member_db_id,
            &chat_db_id,
            &chat_to_member_db_id,
        )
            .await
            .unwrap();
        assert_json_include!(
            actual: json!(result),
            expected: json!({
                "text": "Сделал"
            })
        );
        let existed_values = DictionaryEntity::existed_values(&pool, &chat_db_id).await;
        for expected in ["булочка", "елочка", "батончик"] {
            assert!(existed_values.contains(&expected.to_string()))
        }
    }
}
