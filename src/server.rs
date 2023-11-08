use actix_web::{HttpResponse, post, Responder, web};
use crate::telegram::request::{WebhookRequest};

#[post("/api/telegram")]
async fn telegram_webhook_route(request: web::Json<WebhookRequest>) -> impl Responder {
    println!("{:?}", request.0);
    HttpResponse::Ok()
}