use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::postgres::PgPool;
use tokio::try_join;

use crate::telegram::handler::handle_command;
use crate::telegram::request::RequestPayload;
use crate::telegram::user_service::{bind_user_to_chat, process_chat, process_user};

pub async fn telegram_webhook_route(
    State(pool): State<PgPool>,
    Json(payload): Json<RequestPayload>,
) -> StatusCode {
    if payload.any_message().direct().base.from.is_bot {
        return StatusCode::OK;
    }
    match try_join!(
        process_user(&pool, &payload.any_message().direct().base.from),
        process_chat(&pool, &payload.any_message().direct().base.chat)
    ) {
        Ok((member_id, chat_id)) => {
            bind_user_to_chat(&pool, &member_id, &chat_id).await;
        }
        Err(_) => return StatusCode::OK,
    }
    println!("{:?}", handle_command(&payload));
    StatusCode::OK
}
