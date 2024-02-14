#[cfg(test)]
pub mod helper_functions {
    use axum::body::Body;
    use axum::response::Response;
    use http::Request;
    use sqlx::PgPool;
    use tower::ServiceExt;

    use crate::common::request::RequestPayload;
    use crate::web_app;

    pub async fn make_telegram_request(pool: PgPool, message: &RequestPayload) -> Response<Body> {
        web_app(pool.clone())
            .await
            .oneshot(
                Request::builder()
                    .uri("/api/common")
                    .method(http::Method::POST)
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::to_string(message).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap()
    }
}