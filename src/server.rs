use actix_web::{HttpResponse, post, Responder, web};

use crate::telegram::request::WebhookRequest;
use crate::services::commands::is_command;

#[post("/api/telegram")]
async fn telegram_webhook_route(request: web::Json<WebhookRequest>) -> impl Responder {
    println!("{:?}", is_command(request.0));
    HttpResponse::Ok()
}