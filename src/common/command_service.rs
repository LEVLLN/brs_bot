use std::collections::HashMap;

use once_cell::sync::Lazy;
use rand::Rng;
use sqlx::PgPool;
use unicase::UniCase;

use crate::common::answer_entity_service::{all_keys, substrings, triggers};
use crate::common::command_parser::{
    find_command, parse_command, Command, CommandContainer, CommandSetting, ControlItem,
    COMMAND_SETTING_MAP,
};
use crate::common::db::{ChatId, ChatToMemberId, EntityContentType, MemberId};
use crate::common::error::ProcessError;
use crate::common::lexer::{tokens_to_string, Token};
use crate::common::request::{Message, MessageBody, MessageExt};
use crate::common::response::{
    roll_reply_markup, text_message, text_message_with_roll, BaseBody, ResponseMessage,
};
use crate::common::user_service::{
    morph_answer_chance, pretty_username, random_user_from_chat, set_morph_answer_chance,
    set_substring_answer_chance, substring_answer_chance,
};

static HELP_MAIN: Lazy<String> = Lazy::new(|| {
    String::from("Привет. Я бот и меня зовут Хлебушек.\n\
    Я создан для того, чтобы делать ваши групповые чаты чуточку веселее. \
    Распознаю команды и рандомно и весело отвечаю на сообщения. \
    Достаточно добавить меня в группу и дать доступ на чтение сообщений. \
    Чтобы меня вызвать, нужно придерживаться следующего:\n\n\
    - Конструкция вызова: 'хлеб [команда] [объект для редактирования] [значение]'\n\
    \t> [команда] - обязательный параметр\n\
    \t> [объект для редактирования] - опциональный параметр. Нужен, если этого требует команда.\n\
    \t> [значение] - опциональный параметр. Нужен, если этого требует команда.\n\
    \t> Важно: некоторые команды требуют значение в виде выбранного сообщения в качестве ответа.\n\n\
    - Посмотреть весь список команд: 'хлеб хелп команды'.\n\n\
    - Посмотреть детальную информацию о команде: 'хлеб хелп [команда]'\n\n\
    \t- значение для [команда] можно найти из списка\n\n\
    - Посмотреть информацию о механике и терминологии бота: 'хлеб хелп механика'\n\n\
    Продвигать проект и оставлять пожелания можно на boosty: https://boosty.to/levkey/donate")
});

static HELP_INSTRUCTIONS: Lazy<String> = Lazy::new(|| {
    String::from(
        "Основные элементы:\n\nЗначение \
    - это текст, картинка, видео, гифка, стикер или голосовое сообщение, \
    которое можно у меня сохранить и которые я буду подкидывать в момент, \
    когда ты меньше всего этого ожидаешь :)\n\n\
    Ключ - это строка, на которую закрепили определенное значение\n\n\
    Триггер - это целая строка в тексте сообщения, на которую я 100% среагирую \
    и пришлю тебе сохраненное на эту строку `значение`\n\n\
    Подстрока - это часть строки, которое есть в тексте сообщения. \
    Я сработаю с определенным `процентом срабатывания` \
    и пришлю тебе сохраненное на эту часть строки `значение`\n\n\
    Бред - это должно быть очень смешное слово, на которое \
    я по команде или с определенным `процентом срабатывания` меняю изначальные слова \
    у сообщения в случайной последовательности.\n\n\
    Процент срабатывания - это тот процент, который задает частоту \
    моего автоматического срабатывания на `подстроки` или `бред`. \
    Для `бреда` и `подстроки` можно задать разные проценты. По-умолчанию - это 15%\n\n\
    Алиас - это псевдонимы команд. То есть, у одной команды может быть несколько псевдонимов, \
    по которым можно её вызвать. Это придумано для удобства\n\n\
    Объект редактирования - это обобщенное название `ключу`, `триггеру`, `бреду` и `подстроке`. \
    В некоторых командах их стоит перечислить для операций добавления или удаления",
    )
});

fn command_details_help(command_setting: &CommandSetting) -> String {
    String::new()
        + &format!(
            "Команда: \"{command_name}\". {description}\n\n\
    - Алиасы: [\"{aliases}\"]\n\
    - Объекты редактирования: {available_control_items}\n\
    - Значения обязательны: {required_value}\n\
    - Значение в виде ответа на сообщение: {required_reply}\n\
    - Несколько значений через \",\" и \"или\": {split_values}",
            command_name = command_setting.aliases[0],
            description = command_setting.description,
            aliases = command_setting.aliases.join("\", \""),
            available_control_items = match &command_setting.available_control_items {
                None => {
                    String::from("-")
                }
                Some(_control_items) => {
                    let control_items_info = _control_items
                        .iter()
                        .map(|x| x.name())
                        .collect::<Vec<String>>()
                        .join(", ");
                    if let Some(default_control_item) = &command_setting.default_control_item {
                        control_items_info
                            + &(String::from(". Объект по-умолчанию: ")
                                + &default_control_item.name())
                    } else {
                        control_items_info + ". Указание объекта обязательно"
                    }
                }
            },
            required_value = if command_setting.required_value {
                "Да"
            } else {
                "Нет"
            },
            required_reply = if command_setting.required_reply {
                "Да"
            } else {
                "Нет"
            },
            split_values = if command_setting.split_values {
                "Да"
            } else {
                "Нет"
            },
        )
}

static COMMAND_HELP_MAP: Lazy<HashMap<&'static Command, String>> = Lazy::new(|| {
    let mut result = HashMap::new();
    COMMAND_SETTING_MAP
        .iter()
        .for_each(|(&command, command_setting)| {
            result.insert(command, command_details_help(command_setting));
        });
    result
});

static COMMANDS_HELP_LIST: Lazy<String> = Lazy::new(|| {
    COMMAND_SETTING_MAP.iter().enumerate().fold(
        String::new(),
        |mut output, (index, (_, command_setting))| {
            output += &format!(
                "{number}) {command_name}: {description}\n",
                number = index + 1,
                command_name = command_setting.aliases[0],
                description = command_setting.description,
            );
            output
        },
    ) + "\nЧтобы узнать подробнее о нужной команде необходимо написать: \"хлеб help [команда]\""
});

fn help<'a>(
    command_container: &CommandContainer<'a>,
    chat_id: i64,
    message_id: i64,
) -> Result<ResponseMessage, ProcessError<'a>> {
    Ok(text_message(
        match command_container.rest {
            [argument, ..] if argument == &Token::Word("механика") => {
                HELP_INSTRUCTIONS.to_owned()
            }
            [argument, ..] if argument == &Token::Word("команды") => {
                COMMANDS_HELP_LIST.to_owned()
            }
            tokens => match find_command(tokens) {
                None => HELP_MAIN.to_owned(),
                Some((command, _, _)) => COMMAND_HELP_MAP.get(command).unwrap().to_owned(),
            },
        },
        chat_id,
        message_id,
    ))
}

static WHO_FORMS: Lazy<HashMap<UniCase<&'static str>, &str>> = Lazy::new(|| {
    HashMap::from([
        (UniCase::new(""), ""),
        (UniCase::new("кто"), ""),
        (UniCase::new("кому"), "ему(ей):"),
        (UniCase::new("кем"), "им(ей):"),
        (UniCase::new("с кем"), "с ним(ней):"),
        (UniCase::new("кого"), "его(eё):"),
        (UniCase::new("у кого"), "у него(неё):"),
        (UniCase::new("про кого"), "про него(неё):"),
        (UniCase::new("о ком"), "о нём(ней):"),
        (UniCase::new("чьё"), "его(её):"),
        (UniCase::new("чье"), "его(её):"),
        (UniCase::new("чья"), "его(её):"),
        (UniCase::new("чей"), "его(её):"),
        (UniCase::new("who"), "его(её):"),
    ])
});

async fn who<'a>(
    pool: &PgPool,
    command_container: &CommandContainer<'a>,
    chat_db_id: &ChatId,
    chat_id: i64,
    message_id: i64,
) -> Result<ResponseMessage, ProcessError<'a>> {
    Ok(text_message_with_roll(
        match (
            tokens_to_string(command_container.rest, true),
            WHO_FORMS
                .get(&UniCase::new(&match command_container.command_aliases {
                    [Token::Word(pretext), Token::Word(question)] => {
                        format!("{pretext} {question}")
                    }
                    [Token::Word(question)] => question.to_string(),
                    _ => "".to_string(),
                }))
                .unwrap()
                .to_string(),
            pretty_username(&random_user_from_chat(pool, chat_db_id).await?),
        ) {
            (rest, pretext, username) if !rest.is_empty() && pretext.is_empty() => {
                username + " " + &rest
            }
            (rest, pretext, username) if !rest.is_empty() && !pretext.is_empty() => {
                rest + " " + &pretext + " " + &username
            }
            (rest, pretext, username) if rest.is_empty() && !pretext.is_empty() => {
                pretext + " " + &username
            }
            (_, _, username) => username,
        },
        chat_id,
        message_id,
    ))
}

async fn answer_chance<'a>(
    pool: &PgPool,
    command_container: &CommandContainer<'a>,
    chat_db_id: &ChatId,
    chat_id: i64,
    message_id: i64,
) -> Result<ResponseMessage, ProcessError<'a>> {
    let control_item = command_container.control_item.unwrap();
    match command_container.rest {
        [] => match control_item {
            ControlItem::Substring => substring_answer_chance(pool, chat_db_id).await,
            ControlItem::MorphWord => morph_answer_chance(pool, chat_db_id).await,
            _ => Err(ProcessError::Feedback {
                message: "Объект редактирования не поддерживается",
            }),
        }
        .map(|x| Ok(text_message(x.to_string(), chat_id, message_id)))?,
        [Token::Word(_answer_chance)] => match _answer_chance.parse::<i16>() {
            Ok(x) if (0..=100).contains(&x) => match control_item {
                ControlItem::Substring => set_substring_answer_chance(pool, chat_db_id, x).await,
                ControlItem::MorphWord => set_morph_answer_chance(pool, chat_db_id, x).await,
                _ => Err(ProcessError::Feedback {
                    message: "Объект редактирования не поддерживается",
                }),
            }
            .map(|_| Ok(text_message("Сделано".to_string(), chat_id, message_id)))?,
            _ => Err(ProcessError::Feedback {
                message: "Указано неверное значение. Должно быть целое число от 0 до 100",
            }),
        },
        _ => Err(ProcessError::Feedback {
            message: "Указано неверное значение. Должно быть целое число от 0 до 100",
        }),
    }
}

async fn check<'a>(
    pool: &PgPool,
    command_container: &CommandContainer<'a>,
    chat_db_id: &ChatId,
    chat_id: i64,
    message_id: i64,
) -> Result<ResponseMessage, ProcessError<'a>> {
    let answer_entities = match command_container.control_item {
        Some(ControlItem::Trigger) => triggers(pool, command_container.rest, chat_db_id).await,
        Some(ControlItem::Substring) => substrings(pool, command_container.rest, chat_db_id).await,
        _ => {
            return Err(ProcessError::Feedback {
                message: "Объект редактирования не поддерживается",
            })
        }
    };
    if answer_entities.is_empty() {
        Err(ProcessError::Feedback {
            message: "Ничего не было найдено",
        })
    } else {
        let random_entity =
            answer_entities[rand::thread_rng().gen_range(0..answer_entities.len())].clone();
        // TODO: Move to ResponseMessage::from_answer_entity function
        match &random_entity.content_type {
            EntityContentType::Text => Ok(text_message_with_roll(
                random_entity.value,
                chat_id,
                message_id,
            )),
            EntityContentType::Voice => Ok(ResponseMessage::Voice {
                base_body: BaseBody {
                    chat_id,
                    reply_to_message_id: Some(message_id),
                    reply_markup: roll_reply_markup(),
                },
                voice: random_entity.value,
                caption: random_entity.description,
            }),
            EntityContentType::Picture => Ok(ResponseMessage::Photo {
                base_body: BaseBody {
                    chat_id,
                    reply_to_message_id: Some(message_id),
                    reply_markup: roll_reply_markup(),
                },
                photo: random_entity.value,
                caption: random_entity.description,
            }),
            EntityContentType::Animation => Ok(ResponseMessage::Animation {
                base_body: BaseBody {
                    chat_id,
                    reply_to_message_id: Some(message_id),
                    reply_markup: roll_reply_markup(),
                },
                animation: random_entity.value,
                caption: random_entity.description,
            }),
            EntityContentType::Video => Ok(ResponseMessage::Video {
                base_body: BaseBody {
                    chat_id,
                    reply_to_message_id: Some(message_id),
                    reply_markup: roll_reply_markup(),
                },
                video: random_entity.value,
                caption: random_entity.description,
            }),
            EntityContentType::VideoNote => Ok(ResponseMessage::VideoNote {
                base_body: BaseBody {
                    chat_id,
                    reply_to_message_id: Some(message_id),
                    reply_markup: roll_reply_markup(),
                },
                video_note: random_entity.value,
            }),
            EntityContentType::Sticker => Ok(ResponseMessage::Sticker {
                base_body: BaseBody {
                    chat_id,
                    reply_to_message_id: Some(message_id),
                    reply_markup: roll_reply_markup(),
                },
                sticker: random_entity.value,
            }),
            EntityContentType::Audio => Ok(ResponseMessage::Audio {
                base_body: BaseBody {
                    chat_id,
                    reply_to_message_id: Some(message_id),
                    reply_markup: roll_reply_markup(),
                },
                audio: random_entity.value,
                caption: random_entity.description,
            }),
            EntityContentType::Document => Ok(ResponseMessage::Document {
                base_body: BaseBody {
                    chat_id,
                    reply_to_message_id: Some(message_id),
                    reply_markup: roll_reply_markup(),
                },
                document: random_entity.value,
                caption: random_entity.description,
            }),
        }
    }
}

async fn show<'a>(
    pool: &PgPool,
    chat_db_id: &ChatId,
    reply_message_body: &MessageBody,
    chat_id: i64,
    message_id: i64,
) -> Result<ResponseMessage, ProcessError<'a>> {
    let found_keys = match &reply_message_body.ext {
        MessageExt::Photo { photo, .. } => {
            all_keys(pool, &photo[0].file_unique_id, chat_db_id, false).await
        }
        MessageExt::Text { text } => all_keys(pool, text, chat_db_id, false).await,
        MessageExt::Audio { audio: content, .. }
        | MessageExt::Document {
            document: content, ..
        }
        | MessageExt::Animation {
            animation: content, ..
        }
        | MessageExt::Sticker { sticker: content }
        | MessageExt::Video { video: content, .. }
        | MessageExt::Voice { voice: content, .. }
        | MessageExt::VideoNote {
            video_note: content,
            ..
        } => all_keys(pool, &content.file_unique_id, chat_db_id, false).await,
    };
    if found_keys.is_empty() {
        return Err(ProcessError::Feedback {message: "Ключей не найдено"})
    };
    Ok(text_message(found_keys.iter()
        .fold(String::new(), |s, k| {
            if s.is_empty() {
                s + k
            } else {
                s + "," + " " + k
            }
        }), chat_id, message_id))
}

pub async fn process_command<'a>(
    tokens: &'a Option<Vec<Token<'a>>>,
    message: &'a Message,
    pool: &PgPool,
    _member_db_id: &MemberId,
    chat_db_id: &ChatId,
    _chat_to_member_db_id: &ChatToMemberId,
    is_roll: bool,
) -> Result<ResponseMessage, ProcessError<'a>> {
    let tokens = match tokens {
        None => return Err(ProcessError::Next),
        Some(_tokens) if _tokens.is_empty() => return Err(ProcessError::Next),
        Some(_tokens) => _tokens,
    };
    let chat_id = message.direct().base.chat.id;
    let message_id = if is_roll {
        message.reply().unwrap().base.message_id
    } else {
        message.direct().base.message_id
    };
    match parse_command(tokens, message.reply().is_some()) {
        Ok(command_container) => match &command_container.command {
            Command::Help => help(&command_container, chat_id, message_id),
            Command::Who => who(pool, &command_container, chat_db_id, chat_id, message_id).await,
            Command::AnswerChance => {
                answer_chance(pool, &command_container, chat_db_id, chat_id, message_id).await
            }
            Command::Show => {
                show(
                    pool,
                    chat_db_id,
                    message.reply().unwrap(),
                    chat_id,
                    message_id,
                )
                .await
            }
            Command::Add => Err(ProcessError::Next),
            Command::Remember => Err(ProcessError::Next),
            Command::Delete => Err(ProcessError::Next),
            Command::Check => {
                check(pool, &command_container, chat_db_id, chat_id, message_id).await
            }
            Command::Say => Err(ProcessError::Next),
            Command::Couple => Err(ProcessError::Next),
            Command::Top => Err(ProcessError::Next),
            Command::Channel => Err(ProcessError::Next),
            Command::RandomChance => Err(ProcessError::Next),
            Command::RandomChoose => Err(ProcessError::Next),
            Command::GenerateNonsense => Err(ProcessError::Next),
            Command::Morph => Err(ProcessError::Next),
            Command::MorphDebug => Err(ProcessError::Next),
            Command::Quote => Err(ProcessError::Next),
            Command::Joke => Err(ProcessError::Next),
            Command::Advice => Err(ProcessError::Next),
        },
        Err(err) => Err(err),
    }
}
