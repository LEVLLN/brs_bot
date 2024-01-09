use axum::http::StatusCode;
use axum::Json;

use crate::telegram::handler::handle_command;
use crate::telegram::request::WebhookRequest;

pub async fn telegram_webhook_route(Json(payload): Json<WebhookRequest>) -> StatusCode {
    println!("{:?}", handle_command(payload));
    StatusCode::OK
}
