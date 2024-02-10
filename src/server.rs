use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use log::warn;
use serde_json::Value;
use sqlx::postgres::PgPool;
use tokio::try_join;

use crate::common::message_handler::handle_command;
use crate::common::request::RequestPayload;
use crate::common::user_service::{bind_user_to_chat, process_chat, process_user};

pub async fn telegram_webhook_route(
    State(pool): State<PgPool>,
    Json(payload): Json<Value>,
) -> StatusCode {
    let request_payload = match serde_json::from_value::<RequestPayload>(payload) {
        Ok(request_payload) => request_payload,
        Err(_) => {
            warn!("Receipt not supported body.");
            return StatusCode::OK;
        }
    };
    if request_payload.any_message().direct().base.from.is_bot {
        return StatusCode::OK;
    }
    match try_join!(
        process_user(&pool, &request_payload.any_message().direct().base.from),
        process_chat(&pool, &request_payload.any_message().direct().base.chat)
    ) {
        Ok((member_id, chat_id)) => {
            match bind_user_to_chat(&pool, &member_id, &chat_id).await {
                Ok(_) => {}
                Err(_) => return StatusCode::OK,
            };
        }
        Err(_) => return StatusCode::OK,
    }
    println!("{:?}", handle_command(&request_payload));
    StatusCode::OK
}
