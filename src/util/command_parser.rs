use std::cmp::Reverse;
use std::collections::HashSet;

use once_cell::sync::OnceCell;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use Command::*;
use Token::*;

use crate::util::lexer::{tokenize, Token};

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

#[derive(Debug, Eq, PartialEq, EnumIter, Hash)]
pub enum ControlItem {
    Substring,
    Trigger,
    MorphWord,
    KeyWord,
}

#[derive(Debug, PartialEq)]
pub struct CommandProperty<'a> {
    pub command: &'a Command,
    pub control_item: Option<ControlItem>,
    pub rest: &'a [Token<'a>],
}

#[derive(Debug, PartialEq)]
pub struct CommandParseError<'a> {
    message: &'a str,
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

fn is_bot_call(token: &Token) -> bool {
    [
        Word("хлеб"),
        Word("хлебушек"),
        Word("bread"),
        Word("bread_bot"),
    ]
    .contains(token)
}

fn control_item_from_token<'a>(
    command: &Command,
    control_token: &'a Token,
) -> Result<Option<ControlItem>, CommandParseError<'a>> {
    match command {
        Show | Add | Delete | Remember => match control_token {
            Word("триггер") | Word("триггеры") => Ok(Some(ControlItem::Trigger)),
            Word("подстроку") | Word("подстроки") => {
                Ok(Some(ControlItem::Substring))
            }
            Word("бред") => Ok(Some(ControlItem::MorphWord)),
            Word("ключ") | Word("ключи") => Ok(Some(ControlItem::KeyWord)),
            _ => Err(CommandParseError {
                message: "Ошибка ввода команды. Команда должна содержать объект для редактирования",
            }),
        },
        _ => Ok(None),
    }
}

fn find_command<'a>(
    tokens: &'a [Token<'a>],
) -> Option<(&'a Command, &'a [Token<'a>], &'a [Token<'a>])> {
    match tokens {
        [bot_call, rest @ ..] if is_bot_call(bot_call) => command_keywords()
            .iter()
            .find(|(_, keywords)| {
                keywords.iter().enumerate().all(|(i, t)| {
                    keywords.len() < rest.len() + 1 && rest.len() > i && &rest[i] == t
                })
            })
            .map(|(command, keywords)| {
                (
                    *command,
                    &rest[0..=keywords.len() - 1],
                    &rest[keywords.len()..=rest.len() - 1],
                )
            }),
        _ => None,
    }
}

pub fn parse_command<'a>(
    tokens: &'a [Token<'_>],
) -> Result<Option<CommandProperty<'a>>, CommandParseError<'a>> {
    find_command(tokens)
        .map(|(command, _, rest_after_command)| {
            if [Say, Help, Check, RandomChoose].contains(command) && rest_after_command.is_empty() {
                Err(CommandParseError {
                    message: "Команда нуждается в указанных значениях для обработки",
                })
            } else {
                match rest_after_command {
                    [control_token, rest_after_control_item @ ..] => {
                        let control_item = control_item_from_token(command, control_token)?;
                        match control_item {
                            Some(x) => Ok(CommandProperty {
                                command,
                                control_item: Some(x),
                                rest: rest_after_control_item,
                            }),
                            None => Ok(CommandProperty {
                                command,
                                control_item: None,
                                rest: rest_after_command,
                            }),
                        }
                    }
                    [] => Ok(CommandProperty {
                        command,
                        control_item: None,
                        rest: rest_after_command,
                    }),
                }
            }
        })
        .map_or(Ok(None), |r| r.map(Some))
}

#[cfg(test)]
mod tests {
    use Command::*;

    use crate::util::command_parser::{
        is_bot_call, parse_command, Command, CommandParseError, CommandProperty, ControlItem,
    };
    use crate::util::lexer::{tokenize, Token};

    #[test]
    fn test_is_bot_call() {
        for (input, output) in [
            (Token::Word("хлеб"), true),
            (Token::Word("Хлеб"), true),
            (Token::Word("Хлебушек"), true),
            (Token::Word("хлебушек"), true),
            (Token::Word("bread_bot"), true),
            (Token::Word("bread"), true),
            (Token::Word("BREAD_BOT"), true),
            (Token::Word("хлебушкек"), false),
            (Token::Newline, false),
            (Token::Punctuation("-"), false),
        ] {
            assert_eq!(is_bot_call(&input), output)
        }
    }
    #[test]
    fn test_to_command() {
        for (input, output) in [
            (
                Some("хлеб проверь"),
                Err(CommandParseError {
                    message: "Команда нуждается в указанных значениях для обработки",
                }),
            ),
            (
                Some("хлеб добавь неподстроку?"),
                Err(CommandParseError {
                    message:
                        "Ошибка ввода команды. Команда должна содержать объект для редактирования",
                }),
            ),
            (
                Some("хлеб добавь подстроку?"),
                Ok(Some(CommandProperty {
                    command: &Add,
                    control_item: Some(ControlItem::Substring),
                    rest: &[Token::Punctuation("?")],
                })),
            ),
            // Commands exists
            (
                Some("хлеб кто булочка?"),
                Ok(Some(CommandProperty {
                    command: &Who,
                    control_item: None,
                    rest: &[Token::Word("булочка"), Token::Punctuation("?")],
                })),
            ),
            (
                Some("хлеб КТО булочка?"),
                Ok(Some(CommandProperty {
                    command: &Who,
                    control_item: None,
                    rest: &[Token::Word("булочка"), Token::Punctuation("?")],
                })),
            ),
            (
                Some("ХЛЕБ кто булочка?"),
                Ok(Some(CommandProperty {
                    command: &Who,
                    control_item: None,
                    rest: &[Token::Word("булочка"), Token::Punctuation("?")],
                })),
            ),
            (
                Some("хлеб who булочка?"),
                Ok(Some(CommandProperty {
                    command: &Who,
                    control_item: None,
                    rest: &[Token::Word("булочка"), Token::Punctuation("?")],
                })),
            ),
            (
                Some("хлеб кто?"),
                Ok(Some(CommandProperty {
                    command: &Who,
                    control_item: None,
                    rest: &[Token::Punctuation("?")],
                })),
            ),
            (
                Some("хлеб кто"),
                Ok(Some(CommandProperty {
                    command: &Who,
                    control_item: None,
                    rest: &[],
                })),
            ),
            (
                Some("хлеб процент срабатывания"),
                Ok(Some(CommandProperty {
                    command: &AnswerChance,
                    control_item: None,
                    rest: &[],
                })),
            ),
            (
                Some("хлеб процент"),
                Ok(Some(CommandProperty {
                    command: &AnswerChance,
                    control_item: None,
                    rest: &[],
                })),
            ),
            // Empty raw_text
            (Some(""), Ok(None)),
            // Wrong raw_text
            (Some("some_wrong_text"), Ok(None)),
            // Only bot-name word
            (Some("хлеб"), Ok(None)),
            (Some("Хлеб"), Ok(None)),
            // Without caption
            (None, Ok(None)),
        ] {
            assert_eq!(parse_command(&tokenize(input.unwrap_or_default())), output);
        }
    }
}
