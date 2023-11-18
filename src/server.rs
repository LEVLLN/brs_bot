use actix_web::{post, web, HttpResponse, Responder};
use crate::services::commands::to_command;

use crate::telegram::request::WebhookRequest;

#[post("/api/telegram")]
async fn telegram_webhook_route(request: web::Json<WebhookRequest>) -> impl Responder {
    println!("{:?}", to_command(request.0));
    HttpResponse::Ok()
}
