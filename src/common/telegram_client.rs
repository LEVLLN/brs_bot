use once_cell::sync::Lazy;
use reqwest::{Client, Error, Response};

use crate::common::response::ResponseMessage;
use crate::config::TELEGRAM_URL;

static TELEGRAM_CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub async fn send_message<'a>(response_message: &ResponseMessage) -> Result<Response, Error> {
    TELEGRAM_CLIENT
        .post(
            TELEGRAM_URL.get().unwrap()
                .join(match response_message {
                    ResponseMessage::Text { .. } => "sendMessage",
                    ResponseMessage::_Photo { .. } => "sendPhoto",
                })
                .unwrap(),
        )
        .json(&response_message)
        .send()
        .await
}
