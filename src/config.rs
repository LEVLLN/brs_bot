use once_cell::sync::Lazy;

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://bread_bot:bread_bot@localhost/bread_bot".to_string())
});
