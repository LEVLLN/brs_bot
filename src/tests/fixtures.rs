#[cfg(test)]
pub mod request_body_fixtures {
    use sqlx::PgPool;

    use crate::common::db::Member;
    use crate::common::request::{
        Chat, Message, MessageBase, MessageBody, MessageExt, RequestPayload, User,
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

    pub fn default_chat() -> Chat {
        Chat {
            id: -333322221111,
            title: Some(String::from("SomeChat")),
            first_name: None,
            last_name: None,
            username: None,
        }
    }

    pub async fn existed_chat_user(pool: &PgPool) -> (User, Chat) {
        let mut user = default_user();
        user.id = EXISTED_USER_ID;
        let mut chat = default_chat();
        chat.id = EXISTED_CHAT_ID;
        let existed_db_member = Member::one_by_member_id(pool, user.id).await.unwrap();
        bind_user_to_chat(
            pool,
            existed_db_member.id,
            chat_by_chat_id(pool, chat.id).await.unwrap().id,
        )
        .await
        .unwrap();
        (user, chat)
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
