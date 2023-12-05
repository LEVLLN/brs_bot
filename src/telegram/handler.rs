use crate::core::command::{to_command, Command};
use crate::telegram::request::WebhookRequest;

fn to_command_inner(request: WebhookRequest) -> Option<Command> {
    to_command(request.any_message().direct().ext.raw_text())
}
pub fn handle_command(request: WebhookRequest) {
    let command = to_command_inner(request);
    println!("{:?}", command);
}

#[cfg(test)]
mod tests {
    use crate::core::command::Command;
    use crate::telegram::handler::to_command_inner;
    use crate::telegram::request::{
        Chat, Content, Message, MessageBase, MessageBody, MessageExt, User, WebhookRequest,
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

    #[test]
    fn test_to_command_inner() {
        for (input, output) in [
            // Commands exists
            (text_ext("хлеб кто булочка?"), Some(Command::Who)),
            // Empty raw_text
            (text_ext(""), None),
            (caption_ext(Some("".to_string())), None),
            // Wrong raw_text
            (text_ext("some wrong text"), None),
            (caption_ext(Some("some wrong text".to_string())), None),
            // Only bot-name word
            (text_ext("хлеб"), None),
            (caption_ext(Some("хлеб".to_string())), None),
            // Without caption
            (caption_ext(None), None),
            // Unable caption field
            (
                MessageExt::Sticker {
                    sticker: Content {
                        file_id: "123".to_string(),
                        file_unique_id: "123".to_string(),
                    },
                },
                None,
            ),
        ] {
            assert_eq!(to_command_inner(build_webhook_request(input)), output);
        }
    }
}
