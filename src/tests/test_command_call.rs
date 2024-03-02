#[cfg(test)]
mod tests {
    use assert_json_diff::assert_json_include;
    use serde_json::json;
    use sqlx::PgPool;
    use crate::common::db::Chat as ChatDB;

    use crate::common::lexer::tokenize;
    use crate::common::message_service::{handle_processor, Processor};
    use crate::common::response::ResponseMessage;
    use crate::tests::helpers::fixtures::{
        db_existed_chat_member, default_origin_direct_text_message, request_existed_chat_user,
    };

    async fn call_command(pool: &PgPool, input_text: &str) -> ResponseMessage {
        let (user, chat) = request_existed_chat_user().await;
        let (member_db_id, chat_db_id, chat_to_member_db_id) = db_existed_chat_member(pool).await;
        handle_processor(
            &Processor::Command,
            &Some(tokenize(input_text)),
            &default_origin_direct_text_message(user, chat, input_text),
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
                actual: json!(call_command(&pool, input).await),
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
                actual: json!(call_command(&pool, input).await),
                expected: json!({"text": output})
            );
        }
    }

    #[sqlx::test(
    migrations = "./migrations",
    fixtures(path = "sqlx_fixtures", scripts("default_chat", "default_user"))
    )]
    async fn test_set_morph_answer_chance(pool: PgPool) {
        call_command(&pool, "хлеб процент бреда 10").await;
        let (_, chat_db_id, _) = db_existed_chat_member(&pool).await;
        assert_eq!(Some(10), ChatDB::morph_answer_chance(&pool, &chat_db_id).await);
    }

    #[sqlx::test(
    migrations = "./migrations",
    fixtures(path = "sqlx_fixtures", scripts("default_chat", "default_user"))
    )]
    async fn test_set_substring_answer_chance(pool: PgPool) {
        call_command(&pool, "хлеб процент 25").await;
        let (_, chat_db_id, _) = db_existed_chat_member(&pool).await;
        assert_eq!(Some(25), ChatDB::substring_answer_chance(&pool, &chat_db_id).await);
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
            assert!(json!(call_command(&pool, input).await)
                .get("text")
                .unwrap()
                .to_string()
                .contains(output));
        }
    }
}
