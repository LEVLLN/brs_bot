use std::fmt::Debug;
use std::iter::Iterator;

use log::info;
use sqlx::PgPool;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::common::command_service::process_command;
use crate::common::db::{ChatId, ChatToMemberId, MemberId};
use crate::common::error::ProcessError;
use crate::common::lexer::{tokenize, Token};
use crate::common::request::RequestPayload;
use crate::common::response::{BaseBody, LinkPreviewOption, ResponseMessage};
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
enum Processor {
    Command,
    AutoTrigger,
    AutoSubstring,
    AutoMorph,
}

impl Processor {
    async fn handle<'a>(
        &self,
        tokens: &'a Option<Vec<Token<'a>>>,
        request_payload: &'a RequestPayload,
        pool: &PgPool,
        member_db_id: &MemberId,
        chat_db_id: &ChatId,
        chat_to_member_db_id: &ChatToMemberId,
    ) -> Result<ResponseMessage, ProcessError<'a>> {
        match self {
            Processor::Command => match tokens {
                Some(_tokens) if !_tokens.is_empty() => {
                    process_command(
                        _tokens,
                        request_payload.any_message(),
                        pool,
                        member_db_id,
                        chat_db_id,
                        chat_to_member_db_id,
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
    let (member_db_id, chat_db_id, chat_to_member_db_id) = match process_user_and_chat(
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
    for processor in Processor::iter() {
        match processor
            .handle(
                tokens,
                request_payload,
                pool,
                &member_db_id,
                &chat_db_id,
                &chat_to_member_db_id,
            )
            .await
        {
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
                        &ResponseMessage::Text {
                            base_body: BaseBody {
                                chat_id: request_payload.any_message().direct().base.chat.id,
                                reply_to_message_id: Some(
                                    request_payload.any_message().direct().base.message_id,
                                ),
                            },
                            text: message.to_string(),
                            link_preview_options: LinkPreviewOption { is_disabled: false },
                        },
                        &chat_db_id,
                    )
                    .await;
                    break;
                }
                ProcessError::Next => {
                    info!("{:?} was skipped for {:?}, go next", processor, chat_db_id);
                }
            },
        }
    }
}
