use axum::{routing::post, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use crate::common::http::telegram_webhook_route;

use crate::config::DATABASE_URL;

mod config;
mod tests;
mod common;

pub async fn web_app(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/api/telegram", post(telegram_webhook_route))
        .with_state(pool)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&DATABASE_URL)
        .await
        .expect("cannot connect to db");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, web_app(pool).await).await.unwrap();
}
