mod core;
mod server;
mod telegram;
use axum::{routing::post, Router};
use sqlx::postgres::PgPoolOptions;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://bread_bot:bread_bot@localhost/bread_bot").await.expect("cannot connect to db");
    let app = Router::new()
        .route("/api/telegram", post(server::telegram_webhook_route)).with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
