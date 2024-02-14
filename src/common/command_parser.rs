use std::cmp::Reverse;
use std::collections::HashMap;

use once_cell::sync::{Lazy, OnceCell};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use Command::*;
use Token::*;

use crate::common::error::ProcessError;
use crate::common::lexer::{tokenize, Token};

#[derive(Debug, Eq, PartialEq, EnumIter, Hash, Clone)]
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

#[derive(Debug, PartialEq, Clone, Default)]
struct CommandSetting<'a> {
    aliases: Vec<&'a str>,
    description: &'a str,
    allow_many_values: bool,
    available_control_items: Option<Vec<ControlItem>>,
    default_control_item: Option<ControlItem>,
    required_value: bool,
    required_reply: bool,
}

static COMMAND_SETTING_MAP: Lazy<HashMap<&'static Command, CommandSetting<'static>>> =
    Lazy::new(|| {
        let command_settings = HashMap::from([
            (
                &Help,
                CommandSetting {
                    aliases: vec!["хелп", "хлеп", "help"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    required_reply: false,
                },
            ),
            (
                &Who,
                CommandSetting {
                    aliases: vec!["кто", "who"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    required_reply: false,
                },
            ),
            (
                &AnswerChance,
                CommandSetting {
                    aliases: vec!["процент", "процент срабатывания"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    required_reply: false,
                },
            ),
            (
                &Show,
                CommandSetting {
                    aliases: vec!["покажи", "show"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: Some(vec![
                        ControlItem::Trigger,
                        ControlItem::MorphWord,
                        ControlItem::Substring,
                        ControlItem::KeyWord,
                    ]),
                    default_control_item: Some(ControlItem::Substring),
                    required_value: true,
                    required_reply: false,
                },
            ),
            (
                &Add,
                CommandSetting {
                    aliases: vec!["добавь", "add"],
                    description: "",
                    allow_many_values: true,
                    available_control_items: Some(vec![ControlItem::MorphWord]),
                    default_control_item: None,
                    required_value: true,
                    required_reply: false,
                },
            ),
            (
                &Remember,
                CommandSetting {
                    aliases: vec!["запомни", "remember"],
                    description: "",
                    allow_many_values: true,
                    available_control_items: Some(vec![
                        ControlItem::Trigger,
                        ControlItem::Substring,
                    ]),
                    default_control_item: Some(ControlItem::Substring),
                    required_value: true,
                    required_reply: true,
                },
            ),
            (
                &Check,
                CommandSetting {
                    aliases: vec!["проверь", "проверка", "check"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: Some(vec![
                        ControlItem::Trigger,
                        ControlItem::Substring,
                    ]),
                    default_control_item: Some(ControlItem::Substring),
                    required_value: true,
                    required_reply: false,
                },
            ),
            (
                &Say,
                CommandSetting {
                    aliases: vec!["скажи", "say"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: true,
                    required_reply: false,
                },
            ),
            (
                &Delete,
                CommandSetting {
                    aliases: vec!["удали", "delete"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    required_reply: true,
                },
            ),
            (
                &Couple,
                CommandSetting {
                    aliases: vec!["парочка", "пара", "couple"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    required_reply: false,
                },
            ),
            (
                &Top,
                CommandSetting {
                    aliases: vec!["топ", "top"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    required_reply: false,
                },
            ),
            (
                &Channel,
                CommandSetting {
                    aliases: vec!["канал", "channel", "all"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    required_reply: false,
                },
            ),
            (
                &RandomChance,
                CommandSetting {
                    aliases: vec!["вероятность", "шанс", "chance"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    required_reply: false,
                },
            ),
            (
                &RandomChoose,
                CommandSetting {
                    aliases: vec!["выбери", "выбор", "choose"],
                    description: "",
                    allow_many_values: true,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: true,
                    required_reply: false,
                },
            ),
            (
                &GenerateNonsense,
                CommandSetting {
                    aliases: vec!["бред", "давай", "nonsense"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    required_reply: true,
                },
            ),
            (
                &Morph,
                CommandSetting {
                    aliases: vec!["морф", "морфируй", "morph"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: true,
                    required_reply: false,
                },
            ),
            (
                &MorphDebug,
                CommandSetting {
                    aliases: vec!["морф дебаг", "морфируй дебаг", "morph debug"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: true,
                    required_reply: false,
                },
            ),
            (
                &Quote,
                CommandSetting {
                    aliases: vec!["цит", "цитата", "quote"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    required_reply: false,
                },
            ),
            (
                &Joke,
                CommandSetting {
                    aliases: vec!["анекдот", "анек", "joke"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    required_reply: false,
                },
            ),
            (
                &Advice,
                CommandSetting {
                    aliases: vec!["совет", "advice"],
                    description: "",
                    allow_many_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    required_reply: false,
                },
            ),
        ]);
        assert!(Command::iter().all(|key| command_settings.contains_key(&key)));
        command_settings
    });

#[derive(Debug, Eq, PartialEq, EnumIter, Hash, Clone)]
pub enum ControlItem {
    Substring,
    Trigger,
    MorphWord,
    KeyWord,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CommandProperty<'a> {
    pub command: &'a Command,
    pub control_item: Option<ControlItem>,
    pub rest: &'a [Token<'a>],
}

#[derive(Debug, PartialEq)]
pub struct CommandParseError<'a> {
    pub message: &'a str,
}

fn command_keywords<'a>() -> &'static Vec<(&'a Command, Vec<Token<'a>>)> {
    static INSTANCE: OnceCell<Vec<(&Command, Vec<Token>)>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut result: Vec<(&Command, Vec<Token>)> = COMMAND_SETTING_MAP
            .iter()
            .flat_map(|(command, setting)| {
                setting
                    .aliases
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
) -> Result<Option<ControlItem>, ProcessError<'a>> {
    match command {
        Show | Add | Delete | Remember => match control_token {
            Word("триггер") | Word("триггеры") => Ok(Some(ControlItem::Trigger)),
            Word("подстроку") | Word("подстроки") => {
                Ok(Some(ControlItem::Substring))
            }
            Word("бред") => Ok(Some(ControlItem::MorphWord)),
            Word("ключ") | Word("ключи") => Ok(Some(ControlItem::KeyWord)),
            _ => Err(ProcessError::Feedback {
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
) -> Result<Option<CommandProperty<'a>>, ProcessError<'a>> {
    find_command(tokens)
        .map(|(command, _, rest_after_command)| {
            if [Say, Check, RandomChoose].contains(command) && rest_after_command.is_empty() {
                Err(ProcessError::Feedback {
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

    use crate::common::command_parser::{
        is_bot_call, parse_command, Command, CommandProperty, ControlItem,
    };
    use crate::common::error::ProcessError;
    use crate::common::lexer::{tokenize, Token};

    #[test]
    fn test_is_bot_call() {
        [
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
        ]
        .iter()
        .for_each(|(input, output)| {
            assert_eq!(is_bot_call(input), *output);
        })
    }

    #[test]
    fn test_to_command() {
        [
            (
                Some("хлеб проверь"),
                Err(ProcessError::Feedback {
                    message: "Команда нуждается в указанных значениях для обработки",
                }),
            ),
            (
                Some("хлеб проверь нога"),
                Ok(Some(CommandProperty {
                    command: &Check,
                    control_item: None,
                    rest: &[Token::Word("нога")],
                })),
            ),
            (
                Some("хлеб добавь неподстроку?"),
                Err(ProcessError::Feedback {
                    message:
                        "Ошибка ввода команды. Команда должна содержать объект для редактирования",
                }),
            ),
            (
                Some("хлеб добавь бред слово"),
                Ok(Some(CommandProperty {
                    command: &Add,
                    control_item: Some(ControlItem::MorphWord),
                    rest: &[Token::Word("слово")],
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
        ]
        .iter()
        .for_each(|(input, output)| {
            assert_eq!(parse_command(&tokenize(input.unwrap_or_default())), *output);
        });
    }
}
