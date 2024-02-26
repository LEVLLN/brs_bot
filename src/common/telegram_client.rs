use once_cell::sync::Lazy;
use reqwest::Client;

use crate::common::response::ResponseMessage;
use crate::config::TELEGRAM_URL;

static TELEGRAM_CLIENT: Lazy<Client> = Lazy::new(Client::new);

pub async fn send_message<'a>(response_message: &ResponseMessage<'a>) {
    match TELEGRAM_CLIENT
        .post(
            TELEGRAM_URL.get().unwrap()
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
        Ok(x) => {
            // TODO: just return Result<Json, Error>
            println!("{:?}, {:?}", &x.url().to_string(), &x.text().await)
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}
