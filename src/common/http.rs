use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use log::warn;
use serde_json::Value;
use sqlx::postgres::PgPool;

use crate::common::message_service::process_message;
use crate::common::request::RequestPayload;

pub async fn telegram_webhook_route(
    State(pool): State<PgPool>,
    Json(payload): Json<Value>,
) -> StatusCode {
    match serde_json::from_value::<RequestPayload>(payload) {
        Ok(request_payload) => {
            process_message(&pool, &request_payload).await;
            StatusCode::OK
        }
        Err(err) => {
            warn!("Receipt not supported body. {err}");
            StatusCode::OK
        }
    }
}
