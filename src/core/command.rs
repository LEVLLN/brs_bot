use std::cmp::Reverse;
use std::collections::HashSet;

use once_cell::sync::{Lazy, OnceCell};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use Command::*;
use Token::*;

use crate::core::lexer::{tokenize, Token};

static BOT_KEY_WORDS: Lazy<Vec<Token>> =
    Lazy::new(|| vec![Word("хлеб"), Word("хлебушек"), Word("bread_bot")]);

#[derive(Debug, Eq, PartialEq, EnumIter, Hash)]
pub enum Command {
    Help,
    Who,
    AnswerChance,
    Show,
    Add,
    Remember,
    Delete,
    Check,
    Say,
    Couple,
    Top,
    Channel,
    RandomChance,
    RandomChoose,
    GenerateNonsense,
    Morph,
    MorphDebug,
    Quote,
    Joke,
    Advice,
}

#[derive(Debug, PartialEq)]
pub struct CommandProperty<'a> {
    pub command: &'a Command,
    pub command_end_position: usize,
}

fn command_keywords<'a>() -> &'static Vec<(&'a Command, Vec<Token<'a>>)> {
    static INSTANCE: OnceCell<Vec<(&Command, Vec<Token>)>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let property = vec![
            (&Help, vec!["хелп", "хлеп", "help"]),
            (&Who, vec!["кто", "who"]),
            (&AnswerChance, vec!["процент", "процент срабатывания"]),
            (&Show, vec!["покажи", "show"]),
            (&Add, vec!["добавь", "add"]),
            (&Remember, vec!["запомни", "запомни значение", "remember"]),
            (&Delete, vec!["удали", "delete"]),
            (&Check, vec!["проверь", "проверка", "check"]),
            (&Say, vec!["скажи", "say"]),
            (&Couple, vec!["парочка", "пара", "couple"]),
            (&Top, vec!["топ", "top"]),
            (&Channel, vec!["канал", "channel", "all"]),
            (&RandomChance, vec!["вероятность", "шанс", "chance"]),
            (&RandomChoose, vec!["выбери", "выбор", "choose"]),
            (&GenerateNonsense, vec!["бред", "давай", "nonsense"]),
            (&Morph, vec!["морф", "морфируй", "morph"]),
            (
                &MorphDebug,
                vec!["морф дебаг", "морфируй дебаг", "morph debug"],
            ),
            (&Quote, vec!["цит", "цитата", "quote"]),
            (&Joke, vec!["анекдот", "анек", "joke"]),
            (&Advice, vec!["совет", "advice"]),
        ];
        let property_commands = property
            .iter()
            .map(|(command, _)| *command)
            .collect::<HashSet<_>>();
        assert!(Command::iter().all(|x| property_commands.contains(&x)));
        let mut result: Vec<(&Command, Vec<Token>)> = property
            .iter()
            .flat_map(|(command, vec_keywords)| {
                vec_keywords
                    .iter()
                    .map(|keyword| (*command, tokenize(keyword)))
            })
            .collect();
        // Sorting keywords token by len of tokens desc for funnel search
        result.sort_by_key(|(_, c)| Reverse(c.len()));
        result
    })
}

fn bot_key_word<'a>(value: &'a Vec<Token>) -> Option<Token<'a>> {
    BOT_KEY_WORDS.iter().find_map(|token| {
        if !value.is_empty() && token == &value[0] {
            Some(value[0])
        } else {
            None
        }
    })
}
pub fn to_command_property(tokens: Vec<Token>) -> Option<CommandProperty> {
    bot_key_word(&tokens)?;
    command_keywords().iter().find_map(|(command, keywords)| {
        if keywords.iter().enumerate().all(|(i, t)| {
            keywords.len() < tokens.len() && tokens.len() > i + 1 && &tokens[i + 1] == t
        }) {
            Some(CommandProperty {
                command,
                command_end_position: keywords.len(),
            })
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use Command::*;

    use crate::core::command::{
        bot_key_word, command_keywords, to_command_property, Command, CommandProperty,
    };
    use crate::core::lexer::{tokenize, Token};

    #[test]
    fn test_bot_key_word() {
        command_keywords();
        for (input, output) in [
            ("Хлеб кто булочка?", Some(Token::Word("Хлеб"))),
            ("хлеб кто булочка?", Some(Token::Word("хлеб"))),
            ("Хлебушек кто булочка?", Some(Token::Word("Хлебушек"))),
            ("хлебушек кто булочка?", Some(Token::Word("хлебушек"))),
            ("BREAD_BOT кто булочка?", Some(Token::Word("BREAD_BOT"))),
            ("bread_bot кто булочка?", Some(Token::Word("BREAD_BOT"))),
            ("Хлебукек кто булочка?", None),
            ("", None),
            (".", None),
        ] {
            assert_eq!(bot_key_word(&tokenize(input)), output)
        }
    }
    #[test]
    fn test_to_command() {
        for (input, output) in [
            // Commands exists
            (
                Some("хлеб кто булочка?"),
                Some(CommandProperty {
                    command: &Who,
                    command_end_position: 1,
                }),
            ),
            (
                Some("хлеб КТО булочка?"),
                Some(CommandProperty {
                    command: &Who,
                    command_end_position: 1,
                }),
            ),
            (
                Some("ХЛЕБ кто булочка?"),
                Some(CommandProperty {
                    command: &Who,
                    command_end_position: 1,
                }),
            ),
            (
                Some("хлеб who булочка?"),
                Some(CommandProperty {
                    command: &Who,
                    command_end_position: 1,
                }),
            ),
            (
                Some("хлеб процент срабатывания"),
                Some(CommandProperty {
                    command: &AnswerChance,
                    command_end_position: 2,
                }),
            ),
            (
                Some("хлеб процент"),
                Some(CommandProperty {
                    command: &AnswerChance,
                    command_end_position: 1,
                }),
            ),
            // Empty raw_text
            (Some(""), None),
            // Wrong raw_text
            (Some("some_wrong_text"), None),
            // Only bot-name word
            (Some("хлеб"), None),
            (Some("Хлеб"), None),
            // Without caption
            (None, None),
        ] {
            assert_eq!(to_command_property(tokenize(input.unwrap_or_default())), output);
        }
    }
}
