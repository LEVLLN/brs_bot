use crate::core::command::to_command;
use crate::core::lexer::tokenize;
use crate::telegram::request::WebhookRequest;

pub fn handle_command(request: WebhookRequest) {
    let command_property = request
        .any_message()
        .direct()
        .ext
        .raw_text()
        .map(tokenize)
        .and_then(to_command);
    println!("{:?}", command_property);
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
