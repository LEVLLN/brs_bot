mod config;
mod core;
mod server;
mod telegram;
mod tests;

use crate::config::DATABASE_URL;
use axum::{routing::post, Router};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub async fn web_app(pool: Pool<Postgres>) -> Router {
    tracing_subscriber::fmt::init();
    Router::new()
        .route("/api/telegram", post(server::telegram_webhook_route))
        .with_state(pool)
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&DATABASE_URL)
        .await
        .expect("cannot connect to db");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, web_app(pool).await).await.unwrap();
}
