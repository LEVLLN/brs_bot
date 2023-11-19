use actix_web::{App, HttpServer};

mod core;
mod server;
mod telegram;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(server::telegram_webhook_route))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
