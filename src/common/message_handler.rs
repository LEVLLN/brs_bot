use sqlx::PgPool;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::common::command_parser::parse_command;
use crate::common::db::{ChatId, MemberId};
use crate::common::error::ProcessError;
use crate::common::lexer::{tokenize, Token};
use crate::common::request::{MessageBody, RequestPayload};
use crate::common::user_service::process_user_and_chat;

#[derive(Debug)]
#[allow(dead_code)]
struct MessageContainer<'a> {
    tokens: &'a Option<Vec<Token<'a>>>,
    member_db_id: &'a MemberId,
    chat_db_id: &'a ChatId,
    message_id: &'a u32,
    chat_id: &'a i64,
    user_id: &'a i64,
    direct: &'a MessageBody,
    reply: &'a Option<&'a MessageBody>,
}
enum AutoEntityRegime {
    Trigger,
    Substring,
}
async fn process_command<'a>(
    message_container: &'a MessageContainer<'a>,
) -> Result<(), ProcessError<'a>> {
    let tokens = match message_container.tokens {
        None => return Err(ProcessError::Next),
        Some(tokens) => tokens,
    };
    let command_property = match parse_command(tokens) {
        Ok(Some(command_property)) => command_property,
        Ok(None) => return Err(ProcessError::Next),
        Err(err) => return Err(err),
    };
    println!(
        "tokens: {:?}, command_property: {:?}",
        tokens, command_property
    );
    Err(ProcessError::Next)
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
        message_container: &'a MessageContainer<'a>,
    ) -> Result<(), ProcessError<'a>> {
        match self {
            Processor::Command => process_command(message_container).await,
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
    let message_container = MessageContainer {
        tokens,
        member_db_id: &member_db_id,
        chat_db_id: &chat_db_id,
        message_id: &request_payload.any_message().direct().base.message_id,
        chat_id: &request_payload.any_message().direct().base.chat.id,
        user_id: &request_payload.any_message().direct().base.from.id,
        direct: request_payload.any_message().direct(),
        reply: &request_payload.any_message().reply(),
    };
    for process in Processor::iter() {
        match process.resolve(&message_container).await {
            Ok(()) => {
                println!("Handled Successful");
                break;
            }
            Err(error) => match error {
                ProcessError::Stop => {
                    break;
                }
                ProcessError::Next => {
                    println!("Proccess {:?} was skipped, go next", process)
                }
                ProcessError::Feedback { message } => {
                    println!("User sends feedback {:?}", message);
                    break;
                }
            },
        }
    }
}
