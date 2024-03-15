#[cfg(test)]
pub mod functions {
    use axum::body::Body;
    use axum::response::Response;
    use http::Request;
    use sqlx::{query_as, PgPool, Pool, Postgres};
    use tower::ServiceExt;

    use crate::common::request::RequestPayload;
    use crate::web_app;

    pub async fn chat_by_chat_id(
        pool: &Pool<Postgres>,
        chat_id: i64,
    ) -> Option<crate::common::db::Chat> {
        query_as::<_, crate::common::db::Chat>(&format!(
            "SELECT id, chat_id, name FROM chats WHERE chat_id = {chat_id};"
        ))
        .fetch_one(pool)
        .await
        .ok()
    }

    pub async fn api_telegram_request(pool: PgPool, message: &RequestPayload) -> Response<Body> {
        web_app(pool.clone())
            .await
            .oneshot(
                Request::builder()
                    .uri("/api/telegram")
                    .method(http::Method::POST)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_string(message).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap()
    }
}

#[cfg(test)]
pub mod fixtures {
    use sqlx::PgPool;

    use crate::common::db::{ChatId, ChatToMemberId, Member, MemberId};
    use crate::common::request::{
        CallbackQuery, Chat, Message, MessageBase, MessageBody, MessageExt, ReplyMarkup,
        ReplyMarkupButton, RequestPayload, User,
    };
    use crate::common::user_service::bind_user_to_chat;
    use crate::tests::helpers::functions::chat_by_chat_id;

    pub static EXISTED_USER_ID: i64 = 111222332;
    pub static EXISTED_CHAT_ID: i64 = -333322221112;

    pub fn default_user() -> User {
        User {
            id: 111222333,
            is_bot: false,
            first_name: Some(String::from("FirstName")),
            last_name: Some(String::from("LastName")),
            username: Some(String::from("Username")),
        }
    }

    pub fn bot_user() -> User {
        User {
            id: 1991322963,
            is_bot: true,
            first_name: Some(String::from("Хлебушек")),
            username: Some(String::from("bread_hleb_bot")),
            last_name: None,
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

    pub async fn request_existed_chat_user() -> (User, Chat) {
        let mut user = default_user();
        user.id = EXISTED_USER_ID;
        let mut chat = default_chat();
        chat.id = EXISTED_CHAT_ID;
        (user, chat)
    }

    pub async fn db_existed_chat_member(pool: &PgPool) -> (MemberId, ChatId, ChatToMemberId) {
        bind_user_to_chat(
            pool,
            Member::one_by_member_id(pool, EXISTED_USER_ID)
                .await
                .unwrap()
                .id,
            chat_by_chat_id(pool, EXISTED_CHAT_ID).await.unwrap().id,
        )
        .await
        .unwrap()
    }

    pub fn default_origin_direct_text_message(
        user: &User,
        chat: &Chat,
        text: &str,
    ) -> RequestPayload {
        RequestPayload::Origin {
            update_id: 0,
            message: Message::Common {
                direct: MessageBody {
                    base: MessageBase {
                        message_id: 5555,
                        from: user.clone(),
                        chat: chat.clone(),
                        forward_from: None,
                        forward_from_chat: None,
                    },
                    ext: MessageExt::Text {
                        text: String::from(text),
                    },
                },
                reply_markup: None,
            },
        }
    }

    pub fn roll_callback_message(user: &User, chat: &Chat, reply_text: &str) -> RequestPayload {
        RequestPayload::Callback {
            update_id: 4,
            callback_query: CallbackQuery {
                id: "1".to_string(),
                from: user.clone(),
                message: Message::Replied {
                    direct: MessageBody {
                        base: MessageBase {
                            message_id: 3,
                            from: bot_user(),
                            chat: chat.clone(),
                            forward_from: None,
                            forward_from_chat: None,
                        },
                        ext: MessageExt::Text {
                            text: "Some Text".to_string(),
                        },
                    },
                    reply: Box::new(MessageBody {
                        base: MessageBase {
                            message_id: 2,
                            from: user.clone(),
                            chat: chat.clone(),
                            forward_from: None,
                            forward_from_chat: None,
                        },
                        ext: MessageExt::Text {
                            text: reply_text.to_string(),
                        },
                    }),
                    reply_markup: Some(ReplyMarkup {
                        inline_keyboard: vec![vec![ReplyMarkupButton {
                            text: "Roll".to_string(),
                            callback_data: "".to_string(),
                        }]],
                    }),
                },
            },
        }
    }
}
