use sqlx::PgPool;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::common::command_parser::{parse_command, Command};
use crate::common::db::{ChatId, MemberId};
use crate::common::error::ProcessError;
use crate::common::lexer::{tokenize, Token};
use crate::common::request::{Message, RequestPayload};
use crate::common::user_service::process_user_and_chat;

enum AutoEntityRegime {
    Trigger,
    Substring,
}

async fn process_command<'a>(
    tokens: &'a Vec<Token<'a>>,
    message: &'a Message,
    member_db_id: &MemberId,
    chat_db_id: &ChatId,
) -> Result<(), ProcessError<'a>> {
    let command_property = match parse_command(tokens) {
        Ok(Some(command_property)) => command_property,
        Ok(None) => return Err(ProcessError::Next),
        Err(err) => return Err(err),
    };
    match command_property.command {
        Command::Help => {}
        Command::Who => {}
        Command::AnswerChance => {}
        Command::Show => {}
        Command::Add => {}
        Command::Remember => {}
        Command::Delete => {}
        Command::Check => {}
        Command::Say => {}
        Command::Couple => {}
        Command::Top => {}
        Command::Channel => {}
        Command::RandomChance => {}
        Command::RandomChoose => {}
        Command::GenerateNonsense => {}
        Command::Morph => {}
        Command::MorphDebug => {}
        Command::Quote => {}
        Command::Joke => {}
        Command::Advice => {}
    }
    println!(
        "tokens: {:?}, command_property: {:?}",
        tokens, command_property
    );
    Ok(())
}

async fn process_auto_entity<'a>(regime: AutoEntityRegime) -> Result<(), ProcessError<'a>> {
    match regime {
        AutoEntityRegime::Trigger => Err(ProcessError::Next),
        AutoEntityRegime::Substring => Err(ProcessError::Feedback {
            message: "Function not yet",
        }),
    }
}

async fn process_auto_morph<'a>() -> Result<(), ProcessError<'a>> {
    Err(ProcessError::Next)
}

#[derive(Debug, Eq, PartialEq, EnumIter, Clone)]
enum Processor {
    Command,
    AutoTrigger,
    AutoSubstring,
    AutoMorph,
}

impl Processor {
    async fn resolve<'a>(
        &self,
        tokens: &'a Option<Vec<Token<'a>>>,
        request_payload: &'a RequestPayload,
        member_db_id: &MemberId,
        chat_db_id: &ChatId,
    ) -> Result<(), ProcessError<'a>> {
        match self {
            Processor::Command => match tokens {
                Some(_tokens) if !_tokens.is_empty() => {
                    process_command(
                        _tokens,
                        request_payload.any_message(),
                        member_db_id,
                        chat_db_id,
                    )
                    .await
                }
                _ => Err(ProcessError::Next),
            },
            Processor::AutoTrigger => process_auto_entity(AutoEntityRegime::Trigger).await,
            Processor::AutoSubstring => process_auto_entity(AutoEntityRegime::Substring).await,
            Processor::AutoMorph => process_auto_morph().await,
        }
    }
}

pub async fn process_message<'a>(pool: &PgPool, request_payload: &RequestPayload) {
    let (member_db_id, chat_db_id) = match process_user_and_chat(
        pool,
        &request_payload.any_message().direct().base.from,
        &request_payload.any_message().direct().base.chat,
    )
    .await
    {
        Ok(x) => x,
        Err(_) => {
            return;
        }
    };
    let tokens = &request_payload
        .any_message()
        .direct()
        .ext
        .raw_text()
        .map(tokenize);
    for process in Processor::iter() {
        match process
            .resolve(tokens, request_payload, &member_db_id, &chat_db_id)
            .await
        {
            Ok(()) => {
                println!("{:?} processed successful", process);
                break;
            }
            Err(error) => match error {
                ProcessError::Stop => {
                    break;
                }
                ProcessError::Next => {
                    println!("{:?} was skipped, go next", process)
                }
                ProcessError::Feedback { message } => {
                    println!("User sends feedback {:?}", message);
                    break;
                }
            },
        }
    }
}
