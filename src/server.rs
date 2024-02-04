use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::postgres::PgPool;

use crate::telegram::handler::handle_command;
use crate::telegram::request::RequestPayload;
use crate::telegram::user_service::{process_chat, process_user};

pub async fn telegram_webhook_route(
    State(pool): State<PgPool>,
    Json(payload): Json<RequestPayload>,
) -> StatusCode {
    process_user(&pool, &payload.any_message().direct().base.from).await;
    process_chat(&pool, &payload.any_message().direct().base.chat).await;
    println!("{:?}", handle_command(&payload));
    StatusCode::OK
}
