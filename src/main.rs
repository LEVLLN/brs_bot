mod core;
mod server;
mod telegram;
mod config;

use axum::{routing::post, Router};
use sqlx::postgres::PgPoolOptions;
use crate::config::DATABASE_URL;


#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&DATABASE_URL)
        .await
        .expect("cannot connect to db");
    let app = Router::new()
        .route("/api/telegram", post(server::telegram_webhook_route))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
