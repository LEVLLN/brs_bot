use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::postgres::PgPool;

use crate::telegram::handler::handle_command;
use crate::telegram::request::WebhookRequest;
use crate::telegram::user_service::process_chat;

pub async fn telegram_webhook_route(
    State(pool): State<PgPool>,
    Json(payload): Json<WebhookRequest>,
) -> StatusCode {
    process_chat(&pool, &payload.any_message().direct().base.chat).await.expect("TODO: panic message");
    println!("{:?}", handle_command(&payload));
    StatusCode::OK
}
