use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::services::lexer::{Token, tokenize};
use crate::telegram::request::WebhookRequest;

// TODO: Перенести из проекта bread_bot все команды в структуру Command.
#[derive(Debug, PartialEq, EnumIter)]
enum Command {
    Help,
    Who,
    AnswerChance,
}

// TODO: Использовать lazy_static
impl Command {
    fn key_words(&self) -> Vec<&str> {
        use Command::*;
        match &self {
            Help => vec!["хелп", "help", "помощь"],
            Who => vec!["кто", "who"],
            AnswerChance => vec!["процент срабатывания", "процент"]
        }
    }
}

// TODO: Переписать функцию на to_command, с матчингом в Option<Command>
pub fn is_command(request: WebhookRequest) -> bool {
    request
        .any_message_request()
        .direct()
        .ext
        .raw_text()
        .map(tokenize)
        .and_then(|mut tokens| tokens.pop()) == Some(Token::Text("хлеб"))
}


#[cfg(test)]
mod tests {
    use std::fs;

    use crate::services::commands::is_command;
    use crate::telegram::request::WebhookRequest;

    #[test]
    fn test_with_command_caption() {
        let message: Result<WebhookRequest, _> = serde_json::from_str(fs::read_to_string("tests/resources/text.json").unwrap().as_str());
        is_command(message.unwrap());
    }

    #[test]
    fn test_without_caption() {
        let message: Result<WebhookRequest, _> = serde_json::from_str(fs::read_to_string("tests/resources/animation.json").unwrap().as_str());
        assert!(!is_command(message.unwrap()));
    }
}