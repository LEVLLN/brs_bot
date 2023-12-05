use strum_macros::EnumIter;

use crate::core::lexer::{tokenize, Token};

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

impl<'a> TryFrom<Vec<Token<'a>>> for Command {
    type Error = ();

    fn try_from(value: Vec<Token>) -> Result<Self, Self::Error> {
        use Command::*;
        use Token::*;
        // FIXME: Slice-ы внутри не учитывают правила PartialEq для элементов
        match value[..] {
            [Word("хлеб") | Word("bread_bot"), ..] if value.len() >= 2 => match value[1..] {
                [Word("хелп") | Word("help") | Word("помощь"), ..] => Ok(Help),
                [Word("кто") | Word("who"), ..] => Ok(Who),
                [Word("процент"), Word("срабатывания")] | [Word("процент")] => {
                    Ok(GetAnswerChance)
                }
                [Word("процент"), Word("срабатывания"), ..] | [Word("процент"), ..] => {
                    Ok(SetAnswerChance)
                }
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

pub fn to_command(raw_text: Option<&str>) -> Option<Command> {
    raw_text
        .map(tokenize)
        .and_then(|tokens| Command::try_from(tokens).ok())
}

#[cfg(test)]
mod tests {
    use crate::core::command::{to_command, Command};

    #[test]
    fn test_to_command() {
        for (input, output) in [
            // Commands exists
            (Some("хлеб кто булочка?"), Some(Command::Who)),
            (Some("хлеб who булочка?"), Some(Command::Who)),
            (
                Some("хлеб процент срабатывания"),
                Some(Command::GetAnswerChance),
            ),
            (Some("хлеб процент"), Some(Command::GetAnswerChance)),
            (
                Some("хлеб процент срабатывания 10"),
                Some(Command::SetAnswerChance),
            ),
            (Some("хлеб процент 10)"), Some(Command::SetAnswerChance)),
            // Empty raw_text
            (Some(""), None),
            // Wrong raw_text
            (Some("some_wrong_text"), None),
            // Only bot-name word
            (Some("хлеб"), None),
            // Without caption
            (None, None),
        ] {
            assert_eq!(to_command(input), output);
        }
    }
}
