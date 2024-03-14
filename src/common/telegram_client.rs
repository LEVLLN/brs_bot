use crate::common::db::ChatId;
use log::{info, warn};
use once_cell::sync::Lazy;
use reqwest::Client;

use crate::common::response::ResponseMessage;
use crate::config::TELEGRAM_URL;

static TELEGRAM_CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub async fn send_message<'a>(response_message: &ResponseMessage, chat_db_id: &ChatId) {
    match TELEGRAM_CLIENT
        .post(
            TELEGRAM_URL
                .get()
                .unwrap()
                .join(match response_message {
                    ResponseMessage::Text { .. } => "sendMessage",
                    ResponseMessage::Photo { .. } => "sendPhoto",
                    ResponseMessage::Sticker { .. } => "sendSticker",
                    ResponseMessage::Video { .. } => "sendVideo",
                    ResponseMessage::VideoNote { .. } => "sendVideoNote",
                    ResponseMessage::Voice { .. } => "sendVoice",
                    ResponseMessage::Audio { .. } => "sendAudio",
                    ResponseMessage::Document { .. } => "sendDocument",
                    ResponseMessage::Animation { .. } => "sendAnimation",
                })
                .unwrap(),
        )
        .json(&response_message)
        .send()
        .await
    {
        Ok(response) => match response.status() {
            status_code if status_code == 200 => {
                info!("Message success sent for {:?}", chat_db_id)
            }
            _ => {
                warn!(
                    "Send message failed: {:?} for {:?}. response_message: {:?}",
                    response.text().await,
                    chat_db_id,
                    response_message
                );
            }
        },
        Err(err) => {
            warn!(
                "Send message failed: {} for {:?}. response_message: {:?}",
                err, chat_db_id, response_message
            )
        }
    }
}
