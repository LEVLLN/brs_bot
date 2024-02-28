#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use http::StatusCode;
    use sqlx::PgPool;
    use wiremock::matchers::{
        body_partial_json, body_partial_json_string, body_string_contains, method, path,
    };
    use wiremock::{Mock, ResponseTemplate};

    use crate::tests::fixtures::request_body_fixtures::{default_chat, default_origin_direct_text_message, default_user, EXISTED_CHAT_ID, existed_chat_user};
    use crate::tests::helpers::functions::{api_telegram_request, mock_telegram_server};
    
    #[sqlx::test(migrations = "./migrations")]
    async fn test_skip_all_handlers(pool: PgPool) {
        Mock::given(method("POST"))
            .and(path("/sendMessage"))
            .respond_with(ResponseTemplate::new(200))
            .expect(..=0)
            .mount(&mock_telegram_server().await)
            .await;
        assert_eq!(
            api_telegram_request(
                pool,
                &default_origin_direct_text_message(
                    default_user(),
                    default_chat(),
                    "common message",
                ),
            )
            .await
            .status(),
            StatusCode::OK
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(path = "sqlx_fixtures", scripts("default_chat", "default_user"))
    )]
    async fn test_who(pool: PgPool) {
        let server = mock_telegram_server().await;
        for (input, output) in [
            ("хлеб кто собака", "FirstName LastName собака"),
            ("хлеб кто динозавр?", "FirstName LastName динозавр"),
            ("хлеб кто", "FirstName LastName"),
        ] {
            let (user, chat) = existed_chat_user(&pool).await;
            Mock::given(method("POST"))
                .and(path("/sendMessage"))
                .and(body_partial_json(HashMap::from([
                    ("reply_to_message_id", 5555i64),
                    ("chat_id", EXISTED_CHAT_ID),
                ])))
                .and(body_partial_json(HashMap::from([("text", output)])))
                .respond_with(ResponseTemplate::new(200))
                .expect(1..=1)
                .mount(&server)
                .await;
            assert_eq!(
                api_telegram_request(
                    pool.clone(),
                    &default_origin_direct_text_message(user, chat, input),
                )
                .await
                .status(),
                StatusCode::OK
            );
        }
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_help(pool: PgPool) {
        let server = mock_telegram_server().await;
        for (input, output) in [
            ("хлеб хелп", "Привет. Я бот и меня зовут Хлебушек."),
            ("хлеб хелп команды", "цит:"),
            ("хлеб хелп механика", "Основные элементы"),
            ("хлеб хелп бред", r#"Команда: \"бред\""#),
        ] {
            Mock::given(method("POST"))
                .and(path("/sendMessage"))
                .and(body_partial_json_string("{\"chat_id\": -333322221111}"))
                .and(body_partial_json_string("{\"reply_to_message_id\": 5555}"))
                .and(body_partial_json_string(
                    "{\"link_preview_options\": {\"is_disabled\": true}}",
                ))
                .and(body_string_contains(output))
                .respond_with(ResponseTemplate::new(200))
                .expect(1..=1)
                .mount(&server)
                .await;
            assert_eq!(
                api_telegram_request(
                    pool.clone(),
                    &default_origin_direct_text_message(default_user(), default_chat(), input),
                )
                .await
                .status(),
                StatusCode::OK
            );
        }
    }
}
