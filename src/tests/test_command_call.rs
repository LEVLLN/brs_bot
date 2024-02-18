#[cfg(test)]
mod tests {
    use http::StatusCode;
    use sqlx::PgPool;
    use crate::tests::fixtures::request_body_fixtures::{default_chat, default_origin_direct_text_message, default_user};
    use crate::tests::helpers::functions::make_telegram_request;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_command_call(pool: PgPool) {
        assert_eq!(
            make_telegram_request(
                pool.clone(),
                &default_origin_direct_text_message(default_user(), default_chat(), "хлеб хелп проверь"),
            )
                .await
                .status(),
            StatusCode::OK
        );
    }
}