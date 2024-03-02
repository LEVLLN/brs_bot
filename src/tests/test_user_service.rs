#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use sqlx::{query, PgPool, Row};

    use crate::tests::helpers::fixtures::{
        default_chat, default_origin_direct_text_message, default_user, EXISTED_CHAT_ID,
        EXISTED_USER_ID,
    };
    use crate::tests::helpers::functions::api_telegram_request;

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(path = "sqlx_fixtures", scripts("default_chat"))
    )]
    async fn test_change_chat(pool: PgPool) {
        assert_eq!(
            query("SELECT name from chats where chat_id = $1")
                .bind(EXISTED_CHAT_ID)
                .fetch_one(&pool)
                .await
                .ok()
                .map(|x| x.get::<String, _>("name"))
                .unwrap(),
            String::from("SomeChat"),
        );
        for (title, username, first_name, last_name, expected_name) in [
            (
                Some(String::from("Title")),
                Some(String::from("UserName")),
                Some(String::from("FirstName")),
                Some(String::from("LastName")),
                "Title",
            ),
            (
                None,
                Some(String::from("UserName")),
                Some(String::from("FirstName")),
                Some(String::from("LastName")),
                "UserName",
            ),
            (
                None,
                None,
                Some(String::from("FirstName")),
                Some(String::from("LastName")),
                "FirstName LastName",
            ),
            (
                None,
                None,
                None,
                Some(String::from("LastName")),
                &EXISTED_CHAT_ID.to_string(),
            ),
            (
                None,
                None,
                Some(String::from("FirstName")),
                None,
                &EXISTED_CHAT_ID.to_string(),
            ),
            (None, None, None, None, &EXISTED_CHAT_ID.to_string()),
        ] {
            let mut chat = default_chat();
            chat.id = EXISTED_CHAT_ID;
            chat.title = title;
            chat.username = username;
            chat.first_name = first_name;
            chat.last_name = last_name;
            let message = default_origin_direct_text_message(&default_user(), &chat, "some_text");
            let response = api_telegram_request(pool.clone(), &message).await;
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                query("SELECT name from chats where chat_id = $1")
                    .bind(EXISTED_CHAT_ID)
                    .fetch_one(&pool)
                    .await
                    .ok()
                    .map(|x| x.get::<String, _>("name"))
                    .unwrap(),
                String::from(expected_name),
            );
        }
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(path = "sqlx_fixtures", scripts("default_user"))
    )]
    async fn test_change_user(pool: PgPool) {
        assert_eq!(
            query("SELECT username, last_name, first_name from members where member_id = $1")
                .bind(EXISTED_USER_ID)
                .fetch_one(&pool)
                .await
                .ok()
                .map(|x| (
                    x.get::<String, _>("username"),
                    x.get::<String, _>("first_name"),
                    x.get::<String, _>("last_name")
                ))
                .unwrap(),
            (
                String::from("UserName"),
                String::from("FirstName"),
                String::from("LastName")
            )
        );
        for (username, first_name, last_name, exp_username, exp_first_name, exp_last_name) in [
            (
                Some(String::from("WrongUserName")),
                Some(String::from("WrongFirstName")),
                Some(String::from("WrongLastName")),
                String::from("WrongUserName"),
                String::from("WrongFirstName"),
                String::from("WrongLastName"),
            ),
            (
                Some(String::from("WrongUserName")),
                Some(String::from("WrongFirstName")),
                None,
                String::from("WrongUserName"),
                String::from("WrongFirstName"),
                String::from(""),
            ),
            (
                None,
                Some(String::from("WrongFirstName")),
                None,
                String::from(""),
                String::from("WrongFirstName"),
                String::from(""),
            ),
            (
                None,
                None,
                None,
                String::from(""),
                String::from(""),
                String::from(""),
            ),
        ] {
            let mut user = default_user().clone();
            user.id = EXISTED_USER_ID;
            user.username = username;
            user.first_name = first_name;
            user.last_name = last_name;
            let message = default_origin_direct_text_message(&user, &default_chat(), "some_text");
            let response = api_telegram_request(pool.clone(), &message).await;
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                query("SELECT username, last_name, first_name from members where member_id = $1")
                    .bind(EXISTED_USER_ID)
                    .fetch_one(&pool)
                    .await
                    .ok()
                    .map(|x| (
                        x.get::<String, _>("username"),
                        x.get::<String, _>("first_name"),
                        x.get::<String, _>("last_name")
                    ))
                    .unwrap(),
                (exp_username, exp_first_name, exp_last_name,)
            );
        }
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_user(pool: PgPool) {
        let message =
            default_origin_direct_text_message(&default_user(), &default_chat(), "some_text");
        let chat = &message.any_message().direct().base.chat;
        let user = &message.any_message().direct().base.from;
        assert_eq!(
            query("SELECT chat_id from chats where chat_id = $1")
                .bind(chat.id)
                .fetch_one(&pool)
                .await
                .ok()
                .map(|x| x.get::<i64, _>("chat_id")),
            None
        );
        assert_eq!(
            query("SELECT member_id from members where member_id = $1")
                .bind(user.id)
                .fetch_one(&pool)
                .await
                .ok()
                .map(|x| x.get::<i64, _>("member_id")),
            None
        );
        assert!(!query(
            "SELECT EXISTS (SELECT chats_to_members.* FROM chats_to_members \
            JOIN members ON members.id = chats_to_members.member_id \
            WHERE members.member_id = 111222333);"
        )
        .fetch_one(&pool)
        .await
        .map(|x| x.get::<bool, _>("exists"))
        .unwrap());
        let response = api_telegram_request(pool.clone(), &message).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            query("SELECT chat_id from chats where chat_id = $1")
                .bind(chat.id)
                .fetch_one(&pool)
                .await
                .ok()
                .map(|x| x.get::<i64, _>("chat_id")),
            Some(chat.id)
        );
        assert_eq!(
            query("SELECT member_id from members where member_id = $1")
                .bind(user.id)
                .fetch_one(&pool)
                .await
                .ok()
                .map(|x| x.get::<i64, _>("member_id")),
            Some(user.id)
        );
        assert!(query(
            "SELECT EXISTS (SELECT chats_to_members.* FROM chats_to_members \
            JOIN members ON members.id = chats_to_members.member_id \
            WHERE members.member_id = 111222333);"
        )
        .fetch_one(&pool)
        .await
        .map(|x| x.get::<bool, _>("exists"))
        .unwrap())
    }
}
