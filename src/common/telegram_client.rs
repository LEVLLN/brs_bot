use log::warn;
use once_cell::sync::Lazy;
use reqwest::Client;

use crate::common::response::ResponseMessage;
use crate::config::TELEGRAM_URL;

static TELEGRAM_CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub async fn send_message<'a>(response_message: &ResponseMessage<'a>) {
    if let Err(err) = TELEGRAM_CLIENT
        .post(
            TELEGRAM_URL
                .join(match response_message {
                    ResponseMessage::Text { .. } => "sendMessage",
                    ResponseMessage::Photo { .. } => "sendPhoto",
                })
                .unwrap(),
        )
        .json(&response_message)
        .send()
        .await
    {
        warn!("Message sending error. details: {err}", err = err);
    };
}
