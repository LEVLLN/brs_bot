use std::collections::HashMap;

use once_cell::sync::Lazy;
use sqlx::PgPool;

use crate::common::command_parser::{
    find_command, parse_command, Command, CommandContainer, CommandSetting, COMMAND_SETTING_MAP,
};
use crate::common::db::{ChatId, ChatToMemberId, MemberId};
use crate::common::error::ProcessError;
use crate::common::lexer::{tokens_to_string, Token};
use crate::common::request::{Message, MessageBody};
use crate::common::response::{BaseBody, LinkPreviewOption, ResponseMessage};
use crate::common::user_service::{pretty_username, random_user_from_chat};

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
    let commands_list = COMMAND_SETTING_MAP.iter().enumerate().fold(
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
    );
    commands_list
        + "\nЧтобы узнать подробнее о нужной команде необходимо написать: \"хлеб help [команда]\""
});

fn help<'a>(
    command_container: &CommandContainer<'a>,
    direct_message: &MessageBody,
) -> Result<ResponseMessage, ProcessError<'a>> {
    Ok(ResponseMessage::Text {
        base_body: BaseBody {
            chat_id: direct_message.base.chat.id,
            reply_to_message_id: Some(direct_message.base.message_id),
        },
        text: match command_container.values[0] {
            [argument, ..] if argument == &Token::Word("механика") => {
                HELP_INSTRUCTIONS.to_owned()
            }
            [argument, ..] if argument == &Token::Word("команды") => {
                COMMANDS_HELP_LIST.to_owned()
            }
            tokens => match find_command(tokens) {
                None => HELP_MAIN.to_owned(),
                Some((command, _)) => COMMAND_HELP_MAP.get(command).unwrap().to_owned(),
            },
        },
        link_preview_options: LinkPreviewOption { is_disabled: true },
    })
}

async fn who<'a>(
    pool: &PgPool,
    command_container: &CommandContainer<'a>,
    chat_db_id: &ChatId,
    direct_message: &MessageBody,
) -> Result<ResponseMessage, ProcessError<'a>> {
    Ok(ResponseMessage::Text {
        text: pretty_username(&random_user_from_chat(pool, chat_db_id).await?)
            + &match tokens_to_string(command_container.rest, true) {
                rest if rest.is_empty() => String::from(""),
                rest => String::from(" ") + &rest,
            },
        link_preview_options: LinkPreviewOption { is_disabled: false },
        base_body: BaseBody {
            chat_id: direct_message.base.chat.id,
            reply_to_message_id: Some(direct_message.base.message_id),
        },
    })
}

pub async fn process_command<'a>(
    tokens: &'a [Token<'a>],
    message: &'a Message,
    pool: &PgPool,
    _member_db_id: &MemberId,
    chat_db_id: &ChatId,
    _chat_to_member_db_id: &ChatToMemberId,
) -> Result<ResponseMessage, ProcessError<'a>> {
    match parse_command(tokens, message.reply().is_some()) {
        Ok(command_container) => match &command_container.command {
            Command::Help => help(&command_container, message.direct()),
            Command::Who => who(pool, &command_container, chat_db_id, message.direct()).await,
            Command::AnswerChance => Err(ProcessError::Next),
            Command::Show => Err(ProcessError::Next),
            Command::Add => Err(ProcessError::Next),
            Command::Remember => Err(ProcessError::Next),
            Command::Delete => Err(ProcessError::Next),
            Command::Check => Err(ProcessError::Next),
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
