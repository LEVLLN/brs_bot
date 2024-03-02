use std::cmp::Reverse;
use std::collections::HashMap;

use once_cell::sync::{Lazy, OnceCell};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use Command::*;
use ControlItem::{KeyWord, MorphWord, Substring, Trigger};
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
pub struct CommandSetting<'a> {
    pub aliases: Vec<&'a str>,
    pub description: &'a str,
    pub split_values: bool,
    pub available_control_items: Option<Vec<ControlItem>>,
    pub default_control_item: Option<ControlItem>,
    pub required_value: bool,
    pub get_or_set_value: bool,
    pub required_reply: bool,
}

pub static COMMAND_SETTING_MAP: Lazy<HashMap<&'static Command, CommandSetting<'static>>> =
    Lazy::new(|| {
        let command_settings = HashMap::from([
            (
                &Help,
                CommandSetting {
                    aliases: vec!["help", "хелп", "хлеп", "помощь"],
                    description: "Получить информацию о том, \
                    как пользоваться командами и ботом в целом.",
                    split_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
            (
                &Who,
                CommandSetting {
                    aliases: vec![
                        "кто",
                        "кому",
                        "кем",
                        "с кем",
                        "кого",
                        "у кого",
                        "про кого",
                        "о ком",
                        "чьё",
                        "чье",
                        "чья",
                        "чей",
                        "who",
                    ],
                    description: "Случайно выбирает пользователя группы \
                    и приписывает заданное значение к имени пользователя",
                    split_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
            (
                &AnswerChance,
                CommandSetting {
                    aliases: vec!["процент", "процент срабатывания"],
                    description: "Показ и установка процента автоматического \
                    срабатывания на сущности `бред` и `подстроки`. Чтобы показать значение - \
                    достаточно вызвать команду и указать на какой из доступных объектов это показать, \
                    без целочисленных параметров. Чтобы установить значение, \
                    необходимо добавить параметр из целого числа от 0 до 100",
                    split_values: false,
                    available_control_items: Some(vec![MorphWord, Substring]),
                    default_control_item: Some(Substring),
                    required_value: true,
                    get_or_set_value: true,
                    required_reply: false,
                },
            ),
            (
                &Show,
                CommandSetting {
                    aliases: vec!["покажи", "show"],
                    description: "",
                    split_values: false,
                    available_control_items: Some(vec![Trigger, MorphWord, Substring, KeyWord]),
                    default_control_item: Some(Substring),
                    required_value: false,
                    get_or_set_value: false,
                    required_reply: true,
                },
            ),
            (
                &Add,
                CommandSetting {
                    aliases: vec!["добавь", "add"],
                    description: "",
                    split_values: true,
                    available_control_items: Some(vec![MorphWord]),
                    default_control_item: None,
                    required_value: true,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
            (
                &Remember,
                CommandSetting {
                    aliases: vec!["запомни", "remember"],
                    description: "",
                    split_values: true,
                    available_control_items: Some(vec![Trigger, Substring]),
                    default_control_item: Some(Substring),
                    required_value: true,
                    get_or_set_value: false,
                    required_reply: true,
                },
            ),
            (
                &Check,
                CommandSetting {
                    aliases: vec!["проверь", "проверка", "check"],
                    description: "",
                    split_values: false,
                    available_control_items: Some(vec![Trigger, Substring]),
                    default_control_item: Some(Substring),
                    required_value: true,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
            (
                &Say,
                CommandSetting {
                    aliases: vec!["скажи", "say"],
                    description: "",
                    split_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: true,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
            (
                &Delete,
                CommandSetting {
                    aliases: vec!["удали", "delete"],
                    description: "",
                    split_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    get_or_set_value: false,
                    required_reply: true,
                },
            ),
            (
                &Couple,
                CommandSetting {
                    aliases: vec!["парочка", "пара", "couple"],
                    description: "",
                    split_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
            (
                &Top,
                CommandSetting {
                    aliases: vec!["топ", "top"],
                    description: "",
                    split_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
            (
                &Channel,
                CommandSetting {
                    aliases: vec!["канал", "channel", "all"],
                    description: "",
                    split_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
            (
                &RandomChance,
                CommandSetting {
                    aliases: vec!["вероятность", "шанс", "chance"],
                    description: "",
                    split_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
            (
                &RandomChoose,
                CommandSetting {
                    aliases: vec!["выбери", "выбор", "choose"],
                    description: "",
                    split_values: true,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: true,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
            (
                &GenerateNonsense,
                CommandSetting {
                    aliases: vec!["бред", "давай", "nonsense"],
                    description: "",
                    split_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    get_or_set_value: false,
                    required_reply: true,
                },
            ),
            (
                &Morph,
                CommandSetting {
                    aliases: vec!["морф", "морфируй", "morph"],
                    description: "",
                    split_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: true,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
            (
                &MorphDebug,
                CommandSetting {
                    aliases: vec!["морф дебаг", "морфируй дебаг", "morph debug"],
                    description: "",
                    split_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: true,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
            (
                &Quote,
                CommandSetting {
                    aliases: vec!["цит", "цитата", "quote"],
                    description: "",
                    split_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
            (
                &Joke,
                CommandSetting {
                    aliases: vec!["анекдот", "анек", "joke"],
                    description: "",
                    split_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
            (
                &Advice,
                CommandSetting {
                    aliases: vec!["совет", "advice"],
                    description: "",
                    split_values: false,
                    available_control_items: None,
                    default_control_item: None,
                    required_value: false,
                    get_or_set_value: false,
                    required_reply: false,
                },
            ),
        ]);
        assert!(Command::iter().all(|key| {
            let command_setting = command_settings.get(&key).unwrap();
            match command_setting {
                x if x.aliases.is_empty() => false,
                CommandSetting {
                    available_control_items: Some(x),
                    ..
                } if x.is_empty() => false,
                &CommandSetting {
                    available_control_items: None,
                    default_control_item: Some(_),
                    ..
                } => false,
                &CommandSetting {
                    available_control_items: Some(_),
                    required_value: false,
                    required_reply: false,
                    get_or_set_value: false,
                    ..
                } => false,
                _ => true,
            }
        }));
        command_settings
    });

#[derive(Debug, Eq, PartialEq, EnumIter, Hash, Clone)]
pub enum ControlItem {
    Substring,
    Trigger,
    MorphWord,
    KeyWord,
}

impl ControlItem {
    fn try_from_token<'a>(token: &'a Token<'a>) -> Option<&'a Self> {
        match token {
            x if [Word("триггер"), Word("триггеры")].contains(x) => Some(&Trigger),
            x if [Word("подстроку"), Word("подстроки"), Word("подстрок")].contains(x) => {
                Some(&Substring)
            }
            x if [Word("бред"), Word("бреда")].contains(x) => Some(&MorphWord),
            x if [Word("ключ"), Word("ключи")].contains(x) => Some(&KeyWord),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Substring => String::from("подстроки"),
            Trigger => String::from("триггеры"),
            MorphWord => String::from("бред"),
            KeyWord => String::from("ключи"),
        }
    }
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

pub fn is_bot_call(token: &Token) -> bool {
    [
        Word("хлеб"),
        Word("хлебушек"),
        Word("bread"),
        Word("bread_bot"),
    ]
    .contains(token)
}

#[derive(Debug, PartialEq, Clone)]
pub struct CommandContainer<'a> {
    pub command: &'a Command,
    pub command_aliases: &'a [Token<'a>],
    pub control_item: Option<&'a ControlItem>,
    pub split_values: bool,
    pub rest: &'a [Token<'a>],
}

pub fn find_command<'a>(
    tokens: &'a [Token<'a>],
) -> Option<(&'a Command, &'a [Token<'a>], &'a [Token<'a>])> {
    command_keywords()
        .iter()
        .find(|(_, keywords)| {
            keywords.iter().enumerate().all(|(i, t)| {
                keywords.len() < tokens.len() + 1 && tokens.len() > i && &tokens[i] == t
            })
        })
        .map(|(command, command_aliases)| {
            (
                *command,
                command_aliases.as_slice(),
                &tokens[command_aliases.len()..=tokens.len() - 1],
            )
        })
}

fn find_bot_call_command<'a>(
    tokens: &'a [Token<'a>],
) -> Option<(&'a Command, &'a [Token<'a>], &'a [Token<'a>])> {
    match tokens {
        [bot_call, rest @ ..] if is_bot_call(bot_call) => find_command(rest),
        _ => None,
    }
}

fn negative_control_item_settings<'a>(
    settings: &'a CommandSetting,
    rest: &'a [Token<'a>],
) -> Result<(Option<&'a ControlItem>, &'a [Token<'a>]), ProcessError<'a>> {
    match (
        &settings.available_control_items,
        &settings.default_control_item,
    ) {
        (Some(_), None) => Err(ProcessError::Feedback {
            message: "Необходимо указать объект для редактирования",
        }),
        (Some(_), Some(dci)) => Ok((Some(dci), rest)),
        _ => Ok((None, rest)),
    }
}

fn positive_control_item_settings<'a>(
    settings: &'a CommandSetting,
    control_item: &'a ControlItem,
    rest: &'a [Token<'a>],
) -> Result<(Option<&'a ControlItem>, &'a [Token<'a>]), ProcessError<'a>> {
    match (
        &settings.available_control_items,
        &settings.default_control_item,
    ) {
        (Some(available_control_items), None) if available_control_items.contains(control_item) => {
            Ok((Some(control_item), &rest[1..rest.len()]))
        }
        (Some(available_control_items), Some(default_control_item))
            if !available_control_items.contains(control_item) =>
        {
            Ok((Some(default_control_item), &rest[1..rest.len()]))
        }
        (Some(available_control_items), None)
            if !available_control_items.contains(control_item) =>
        {
            Err(ProcessError::Feedback {
                message: "Указан недопустимый объект для редактирования",
            })
        }
        (None, None) => Ok((None, rest)),
        _ => Ok((Some(control_item), &rest[1..rest.len()])),
    }
}

pub fn parse_command<'a>(
    tokens: &'a [Token<'_>],
    has_reply: bool,
) -> Result<CommandContainer<'a>, ProcessError<'a>> {
    if let Some(validated_command) =
        find_bot_call_command(tokens).map(|(command, command_aliases, rest_after_command)| {
            let settings = COMMAND_SETTING_MAP.get(command).unwrap();
            if settings.required_reply && !has_reply {
                return Err(ProcessError::Feedback {
                    message: "Необходимо выбрать сообщение в ответ",
                });
            }
            match rest_after_command {
                [] => negative_control_item_settings(settings, rest_after_command),
                rest => match ControlItem::try_from_token(&rest[0]) {
                    None => negative_control_item_settings(settings, rest_after_command),
                    Some(control_item) => {
                        positive_control_item_settings(settings, control_item, rest)
                    }
                },
            }
            .and_then(|(control_item, rest)| match rest {
                [] if settings.required_value && !settings.get_or_set_value => {
                    Err(ProcessError::Feedback {
                        message: "Необходимо указать значения",
                    })
                }
                _ => Ok(CommandContainer {
                    command,
                    command_aliases,
                    control_item,
                    split_values: settings.split_values,
                    rest,
                }),
            })
        })
    {
        validated_command
    } else {
        Err(ProcessError::Next)
    }
}
