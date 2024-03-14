use crate::common::command_service::process_command;
use crate::common::db::{ChatId, ChatToMemberId, MemberId};
use crate::common::error::ProcessError;
use crate::common::lexer::tokenize;
use crate::common::request::{ReplyMarkup, ReplyMarkupButton, RequestPayload};
use crate::common::response::ResponseMessage;
use sqlx::PgPool;

pub async fn process_callback<'a>(
    request_payload: &'a RequestPayload,
    pool: &PgPool,
    member_db_id: &MemberId,
    chat_db_id: &ChatId,
    chat_to_member_db_id: &ChatToMemberId,
) -> Result<ResponseMessage, ProcessError<'a>> {
    if request_payload.any_message().reply().is_none() {
        return Err(ProcessError::Next);
    };
    match request_payload.any_message().reply_markup() {
        None => Err(ProcessError::Next),
        Some(ReplyMarkup { inline_keyboard: x }) => match &x[0][0] {
            ReplyMarkupButton { text, .. } if text == "Roll" => {
                let tokens = request_payload
                    .any_message()
                    .reply()
                    .unwrap()
                    .ext
                    .raw_text()
                    .map(tokenize);
                match process_command(
                    &tokens,
                    request_payload.any_message(),
                    pool,
                    member_db_id,
                    chat_db_id,
                    chat_to_member_db_id,
                )
                .await
                {
                    Ok(response_message) => Ok(response_message),
                    Err(_) => Err(ProcessError::Stop),
                }
            }
            _ => Err(ProcessError::Stop),
        },
    }
}
