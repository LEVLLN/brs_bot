use crate::core::command::to_command_property;
use crate::core::lexer::{Token, tokenize};
use crate::telegram::request::WebhookRequest;


pub fn handle_command<'a>(request: WebhookRequest) -> Option<&'a str> {
    let tokens = request
        .any_message()
        .direct()
        .ext
        .raw_text()
        .map(tokenize)?;
    let command_property = to_command_property(tokens);
    println!("{:?}", command_property);
    Some("test")
}

#[cfg(test)]
mod tests {
    use crate::telegram::request::{
        Chat, Message, MessageBase, MessageBody, MessageExt, User, WebhookRequest,
    };

    fn build_webhook_request(ext: MessageExt) -> WebhookRequest {
        WebhookRequest::Origin {
            update_id: 0,
            message: Message::Common {
                direct: MessageBody {
                    base: MessageBase {
                        message_id: 0,
                        from: User {
                            id: 0,
                            is_bot: false,
                            first_name: None,
                            last_name: None,
                            username: None,
                        },
                        chat: Chat {
                            id: 0,
                            title: None,
                            first_name: None,
                            last_name: None,
                            username: None,
                        },
                        forward_from: None,
                        forward_from_chat: None,
                    },
                    ext,
                },
            },
        }
    }
    fn caption_ext(caption: Option<String>) -> MessageExt {
        MessageExt::Photo {
            photo: vec![],
            caption,
        }
    }
    fn text_ext(text: &str) -> MessageExt {
        MessageExt::Text {
            text: text.to_string(),
        }
    }
}
