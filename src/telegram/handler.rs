use crate::core::command::parse_command;
use crate::core::lexer::{tokenize, Token};
use crate::telegram::request::RequestPayload;

pub fn tokens_from_request(request: &RequestPayload) -> Option<Vec<Token>> {
    request
        .any_message()
        .direct()
        .ext
        .raw_text()
        .map(tokenize)
}

pub fn handle_command<'a>(request: &RequestPayload) -> Option<&'a str> {
    let tokens = &tokens_from_request(request)?;
    let command_property = parse_command(tokens);
    println!("{:?}", command_property);
    Some("test")
}

#[cfg(test)]
mod tests {
    use crate::telegram::request::{
        Chat, Message, MessageBase, MessageBody, MessageExt, User, RequestPayload,
    };

    fn build_webhook_request(ext: MessageExt) -> RequestPayload {
        RequestPayload::Origin {
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
