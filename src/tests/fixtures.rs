#[cfg(test)]
pub mod request_body_fixtures {
    use crate::common::request::{
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