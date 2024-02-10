#[cfg(test)]
mod helper_functions {
    use axum::body::Body;
    use axum::response::Response;
    use http::Request;
    use sqlx::PgPool;
    use tower::ServiceExt;

    use crate::telegram::request::RequestPayload;
    use crate::web_app;

    pub async fn make_telegram_request(pool: PgPool, message: &RequestPayload) -> Response<Body> {
        web_app(pool.clone())
            .await
            .oneshot(Request::builder()
                .uri("/api/telegram")
                .method(http::Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(message).unwrap()))
                .unwrap())
            .await
            .unwrap()
    }
}

#[cfg(test)]
mod body_fixtures {
    use crate::telegram::request::{
        Chat, Message, MessageBase, MessageBody, MessageExt, RequestPayload, User,
    };

    pub fn default_user() -> User {
        User {
            id: 111222333,
            is_bot: false,
            first_name: Some(String::from("FirstName")),
            last_name: Some(String::from("LastName")),
            username: Some(String::from("Username")),
        }
    }

    pub fn default_chat() -> Chat {
        Chat {
            id: -333322221111,
            title: Some(String::from("SomeChat")),
            first_name: None,
            last_name: None,
            username: None,
        }
    }
    pub fn default_origin_direct_text_message(
        user: User,
        chat: Chat,
        text: &str,
    ) -> RequestPayload {
        RequestPayload::Origin {
            update_id: 0,
            message: Message::Common {
                direct: MessageBody {
                    base: MessageBase {
                        message_id: 5555,
                        from: user,
                        chat,
                        forward_from: None,
                        forward_from_chat: None,
                    },
                    ext: MessageExt::Text {
                        text: String::from(text),
                    },
                },
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use sqlx::{PgPool, query, Row};

    use crate::tests::test_server::body_fixtures::{
        default_chat, default_origin_direct_text_message, default_user,
    };
    use crate::tests::test_server::helper_functions::make_telegram_request;
    // TODO: Test for change names of existed user
    // TODO: Test for change title of existed chat
    // TODO: Test for skip messages from bot users
    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_user(pool: PgPool) {
        let message =
            default_origin_direct_text_message(default_user(), default_chat(), "some_text");
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
        let response = make_telegram_request(pool.clone(), &message).await;
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
        )
    }
}
