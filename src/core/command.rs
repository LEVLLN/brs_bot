use std::collections::HashMap;

use once_cell::sync::{Lazy, OnceCell};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use Command::*;
use Token::*;

use crate::core::lexer::Token;

type CommandKeyWordsTokens = Vec<Vec<Token<'static>>>;

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

pub fn command_keywords() -> &'static HashMap<&'static Command, CommandKeyWordsTokens> {
    static INSTANCE: OnceCell<HashMap<&Command, CommandKeyWordsTokens>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let result: HashMap<&Command, CommandKeyWordsTokens> = HashMap::from([
            (
                &Help,
                vec![vec![Word("help")], vec![Word("помощь")], vec![Word("хелп")]],
            ),
            (&Who, vec![vec![Word("кто")], vec![Word("who")]]),
            (
                &AnswerChance,
                vec![
                    vec![Word("процент"), Word("срабатывания")],
                    vec![Word("процент")],
                ],
            ),
            (&Show, vec![vec![Word("покажи")], vec![Word("show")]]),
            (&Add, vec![vec![Word("добавь")], vec![Word("add")]]),
            (
                &Remember,
                vec![
                    vec![Word("запомни")],
                    vec![Word("remember")],
                    vec![Word("запомни"), Word("значение")],
                ],
            ),
            (&Delete, vec![vec![Word("удали")], vec![Word("delete")]]),
            (
                &Check,
                vec![
                    vec![Word("проверь")],
                    vec![Word("проверка")],
                    vec![Word("check")],
                ],
            ),
            (&Say, vec![vec![Word("скажи")], vec![Word("say")]]),
            (
                &Couple,
                vec![
                    vec![Word("парочка")],
                    vec![Word("пара")],
                    vec![Word("couple")],
                ],
            ),
            (&Top, vec![vec![Word("топ")], vec![Word("top")]]),
            (
                &Channel,
                vec![
                    vec![Word("channel")],
                    vec![Word("all")],
                    vec![Word("канал")],
                ],
            ),
            (
                &RandomChance,
                vec![
                    vec![Word("вероятность")],
                    vec![Word("шанс")],
                    vec![Word("chance")],
                ],
            ),
            (
                &RandomChoose,
                vec![
                    vec![Word("выбери")],
                    vec![Word("выбор")],
                    vec![Word("choose")],
                ],
            ),
            (
                &GenerateNonsense,
                vec![
                    vec![Word("бред")],
                    vec![Word("давай")],
                    vec![Word("nonsense")],
                ],
            ),
            (
                &Morph,
                vec![
                    vec![Word("морф")],
                    vec![Word("морфируй")],
                    vec![Word("morph")],
                ],
            ),
            (
                &MorphDebug,
                vec![
                    vec![Word("морф"), Word("дебаг")],
                    vec![Word("морфируй"), Word("дебаг")],
                    vec![Word("morph"), Word("debug")],
                ],
            ),
            (
                &Quote,
                vec![vec![Word("цит")], vec![Word("цитата")], vec![Word("quote")]],
            ),
            (
                &Joke,
                vec![
                    vec![Word("анекдот")],
                    vec![Word("анек")],
                    vec![Word("joke")],
                ],
            ),
            (&Advice, vec![vec![Word("совет")], vec![Word("advice")]]),
        ]);
        assert!(
            Command::iter().all(|x| result.contains_key(&x)),
            "Command not contains keywords"
        );
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

pub fn to_command(tokens: Vec<Token>) -> Option<CommandProperty> {
    bot_key_word(&tokens)?;
    command_keywords()
        .iter()
        .find_map(|(command_key, command_collection)| {
            command_collection.iter().find_map(|keyword_tokens| {
                if keyword_tokens.iter().enumerate().all(|(i, t)| {
                    keyword_tokens.len() < tokens.len()
                        && tokens.len() > i + 1
                        && &tokens[i + 1] == t
                }) {
                    Some(CommandProperty {
                        command: command_key,
                        command_end_position: keyword_tokens.len(),
                    })
                } else {
                    None
                }
            })
        })
}

#[cfg(test)]
mod tests {
    use Command::*;

    use crate::core::command::{bot_key_word, to_command, Command, CommandProperty};
    use crate::core::lexer::{tokenize, Token};

    #[test]
    fn test_bot_key_word() {
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
            assert_eq!(to_command(tokenize(input.unwrap_or_default())), output);
        }
    }
}
