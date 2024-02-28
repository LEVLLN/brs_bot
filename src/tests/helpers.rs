#[cfg(test)]
pub mod functions {
    use std::net::TcpListener;

    use axum::body::Body;
    use axum::response::Response;
    use http::Request;
    use once_cell::sync::Lazy;
    use sqlx::{PgPool, Pool, Postgres, query_as};
    use tower::ServiceExt;
    use wiremock::MockServer;

    use crate::common::request::RequestPayload;
    use crate::config::init_telegram_url;
    use crate::web_app;

    static LISTENER: Lazy<TcpListener> = Lazy::new(|| TcpListener::bind("127.0.0.1:5555").unwrap());

    pub async fn mock_telegram_server() -> MockServer {
        let server = MockServer::builder()
            .listener(LISTENER.try_clone().unwrap())
            .start()
            .await;
        init_telegram_url(Some(server.uri()));
        server
    }
    
    pub async fn chat_by_chat_id(pool: &Pool<Postgres>, chat_id: i64) -> Option<crate::common::db::Chat> {
        query_as::<_, crate::common::db::Chat>(&format!(
            "SELECT id, chat_id, name FROM chats WHERE chat_id = {chat_id};"
        ))
            .fetch_one(pool)
            .await
            .ok()
    }
    
    pub async fn api_telegram_request(pool: PgPool, message: &RequestPayload) -> Response<Body> {
        web_app(pool.clone())
            .await
            .oneshot(
                Request::builder()
                    .uri("/api/telegram")
                    .method(http::Method::POST)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_string(message).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap()
    }
}
