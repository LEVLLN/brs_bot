use std::fmt::Debug;
use std::iter::Iterator;

use crate::common::callback_service::process_callback;
use log::{info};
use sqlx::PgPool;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::common::command_service::process_command;
use crate::common::db::{ChatId, ChatToMemberId, MemberId};
use crate::common::error::ProcessError;
use crate::common::lexer::{tokenize, Token};
use crate::common::request::RequestPayload;
use crate::common::response::{text_message, ResponseMessage};
use crate::common::telegram_client::send_message;
use crate::common::user_service::process_user_and_chat;

enum AutoEntityRegime {
    Trigger,
    Substring,
}

async fn process_auto_entity<'a>(
    regime: AutoEntityRegime,
) -> Result<ResponseMessage, ProcessError<'a>> {
    match regime {
        AutoEntityRegime::Trigger => Err(ProcessError::Next),
        AutoEntityRegime::Substring => Err(ProcessError::Next),
    }
}

async fn process_auto_morph<'a>() -> Result<ResponseMessage, ProcessError<'a>> {
    Err(ProcessError::Next)
}

#[derive(Debug, Eq, PartialEq, EnumIter, Clone)]
pub enum Processor {
    Callback,
    Command,
    AutoTrigger,
    AutoSubstring,
    AutoMorph,
}

pub async fn handle_processor<'a>(
    processor: &Processor,
    tokens: &'a Option<Vec<Token<'a>>>,
    request_payload: &'a RequestPayload,
    pool: &PgPool,
    member_db_id: &MemberId,
    chat_db_id: &ChatId,
    chat_to_member_db_id: &ChatToMemberId,
) -> Result<ResponseMessage, ProcessError<'a>> {
    match processor {
        Processor::Command => {
            process_command(
                tokens,
                request_payload.any_message(),
                pool,
                member_db_id,
                chat_db_id,
                chat_to_member_db_id,
                false,
            )
            .await
        }
        Processor::AutoTrigger => process_auto_entity(AutoEntityRegime::Trigger).await,
        Processor::AutoSubstring => process_auto_entity(AutoEntityRegime::Substring).await,
        Processor::AutoMorph => process_auto_morph().await,
        Processor::Callback => {
            process_callback(
                request_payload,
                pool,
                member_db_id,
                chat_db_id,
                chat_to_member_db_id,
            )
            .await
        }
    }
}
pub async fn process_message<'a>(pool: &PgPool, request_payload: &RequestPayload) {
    let (member_db_id, chat_db_id, chat_to_member_db_id) = match process_user_and_chat(
        pool,
        &request_payload.any_message().direct().base.from,
        &request_payload.any_message().direct().base.chat,
        &request_payload.any_message().reply_markup().is_some(),
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
    for processor in Processor::iter() {
        let response_message = handle_processor(
            &processor,
            tokens,
            request_payload,
            pool,
            &member_db_id,
            &chat_db_id,
            &chat_to_member_db_id,
        )
        .await;
        match response_message {
            Ok(response_message) => {
                info!("{:?} success completed for {:?}", processor, chat_db_id);
                send_message(&response_message, &chat_db_id).await;
                break;
            }
            Err(error) => match error {
                ProcessError::Stop => {
                    info!("{:?} was stopped for {:?}", processor, chat_db_id);
                    break;
                }
                ProcessError::Feedback { message } => {
                    info!(
                        "User error, sends feedback: {:?} for {:?}",
                        message, chat_db_id
                    );
                    send_message(
                        &text_message(
                            message.to_string(),
                            request_payload.any_message().direct().base.chat.id,
                            request_payload.any_message().direct().base.message_id,
                        ),
                        &chat_db_id,
                    )
                    .await;
                    break;
                }
                ProcessError::Next => {
                    info!("{:?} was skipped for {:?}, go next", processor, chat_db_id);
                    continue;
                }
            },
        }
    }
}
