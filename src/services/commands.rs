use std::str::FromStr;

use strum_macros::EnumIter;

use crate::services::lexer::{tokenize, Token};
use crate::telegram::request::WebhookRequest;

// TODO: Перенести из проекта bread_bot все команды в структуру Command.
// Задуматься над хранением внутри экземпляров Enum-а дополнительный тип CommandProperty
// Либо с разнесением в него rest_text после токенизации команды,
// Либо указатель на raw_text, где заканчивается команда
#[derive(Debug, PartialEq, EnumIter)]
pub enum Command {
    Help,
    Who,
    GetAnswerChance,
    SetAnswerChance,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "хелп" | "help" | "помощь" => Ok(Command::Help),
            "кто" | "who" => Ok(Command::Who),
            "процент срабатывания" | "процент" => {
                Ok(Command::SetAnswerChance)
            }
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<Vec<Token<'a>>> for Command {
    type Error = ();

    fn try_from(value: Vec<Token>) -> Result<Self, Self::Error> {
        use Command::*;
        use Token::*;
        // FIXME: Slice-ы внутри не учитывают правила PartialEq для элементов
        match &value.as_slice() {
            [Text("хлеб") | Text("bread_bot"), ..] if value.len() >= 2 => {
                match value[1..=value.len() - 1] {
                    [Text("хелп") | Text("help") | Text("помощь"), ..] => Ok(Help),
                    [Text("кто") | Text("who"), ..] => Ok(Who),
                    [Text("процент"), Text("срабатывания")] | [Text("процент")] => {
                        Ok(GetAnswerChance)
                    }
                    [Text("процент"), Text("срабатывания"), ..] | [Text("процент"), ..] => {
                        Ok(SetAnswerChance)
                    }
                    _ => Err(()),
                }
            }
            _ => Err(()),
        }
    }
}
pub fn to_command(request: WebhookRequest) -> Option<Command> {
    request
        .any_message()
        .direct
        .ext
        .raw_text()
        .map(tokenize)
        .and_then(|command| match Command::try_from(command) {
            Ok(command) => Some(command),
            Err(_) => None,
        })
}

#[cfg(test)]
mod tests {
    use crate::services::commands::{to_command, Command};
    use crate::telegram::request::{
        Chat, Content, Message, MessageBase, MessageExt, MessagePart, User, WebhookRequest,
    };

    fn build_webhook_request(ext: MessageExt) -> WebhookRequest {
        WebhookRequest::Origin {
            update_id: 0,
            message: Message {
                direct: MessagePart {
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
                reply: None,
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
    fn test_command_set() {
        assert_eq!(
            to_command(build_webhook_request(text_ext("хлеб кто булочка?"))),
            Some(Command::Who),
        );
        assert_eq!(
            to_command(build_webhook_request(caption_ext(Some(
                "хлеб who булочка?".to_string()
            )))),
            Some(Command::Who),
        );
        assert_eq!(
            to_command(build_webhook_request(text_ext("хлеб процент срабатывания"))),
            Some(Command::GetAnswerChance),
        );
        assert_eq!(
            to_command(build_webhook_request(caption_ext(Some(
                "хлеб процент".to_string()
            )))),
            Some(Command::GetAnswerChance),
        );
        assert_eq!(
            to_command(build_webhook_request(text_ext(
                "хлеб процент срабатывания 10"
            ))),
            Some(Command::SetAnswerChance),
        );
        assert_eq!(
            to_command(build_webhook_request(caption_ext(Some(
                "хлеб процент 10".to_string()
            )))),
            Some(Command::SetAnswerChance),
        );
    }

    #[test]
    fn test_command_empty_text() {
        assert_eq!(to_command(build_webhook_request(text_ext(""))), None);
    }

    #[test]
    fn test_command_wrong_text() {
        assert_eq!(
            to_command(build_webhook_request(text_ext("some wrong text"))),
            None
        );
    }

    #[test]
    fn test_command_only_bot_name() {
        assert_eq!(to_command(build_webhook_request(text_ext("хлеб"))), None);
        assert_eq!(
            to_command(build_webhook_request(caption_ext(Some("хлеб".to_string())))),
            None
        );
    }
    #[test]
    fn test_caption_is_none() {
        assert_eq!(to_command(build_webhook_request(caption_ext(None))), None);
    }

    #[test]
    fn test_caption_is_empty() {
        assert_eq!(
            to_command(build_webhook_request(caption_ext(Some("".to_string())))),
            None
        );
    }

    #[test]
    fn test_caption_unable_field() {
        assert_eq!(
            to_command(build_webhook_request(MessageExt::Sticker {
                sticker: Content {
                    file_id: "123".to_string(),
                    file_unique_id: "123".to_string()
                }
            })),
            None
        );
    }
}
