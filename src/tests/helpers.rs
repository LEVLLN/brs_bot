#[cfg(test)]
pub mod functions {
    use axum::body::Body;
    use axum::response::Response;
    use http::Request;
    use once_cell::sync::OnceCell;
    use sqlx::{query_as, PgPool, Pool, Postgres};
    use tower::ServiceExt;
    use wiremock::MockServer;

    use crate::common::request::RequestPayload;
    use crate::config::init_telegram_url;
    use crate::web_app;

    pub async fn mock_telegram_server() -> &'static MockServer {
        static INSTANCE: OnceCell<MockServer> = OnceCell::new();
        let server = MockServer::start().await;
        init_telegram_url(Some(server.uri()));
        INSTANCE.get_or_init(|| server)
    }

    pub async fn chat_by_chat_id(
        pool: &Pool<Postgres>,
        chat_id: i64,
    ) -> Option<crate::common::db::Chat> {
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
