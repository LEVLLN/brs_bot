use once_cell::sync::Lazy;
use url::Url;

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    let database_str = match std::env::var("DATABASE_URL") {
        Ok(database_url) => database_url,
        Err(_) => panic!("DATABASE_URL environment variable does not exists!"),
    };
    let url = Url::parse(&database_str).expect("Not valid database url");
    assert_ne!(url.username(), "", "Username does not exists");
    assert_ne!(url.host(), None, "Host does not exists");
    assert_ne!(url.password(), None, "Password does not exists");
    assert_ne!(url.path(), "/", "Database name does not exists");
    assert_eq!(url.scheme(), "postgres", "Url need protocol: postgres");
    database_str
});

static TELEGRAM_BOT_TOKEN: Lazy<String> = Lazy::new(|| {
    format!(
        "/{bot_id}:{bot_token}/",
        bot_id = match std::env::var("TELEGRAM_BOT_ID") {
            Ok(bot_id) => bot_id,
            Err(_) => panic!("TELEGRAM_BOT_ID environment variable does not exists!"),
        },
        bot_token = match std::env::var("TELEGRAM_BOT_TOKEN") {
            Ok(bot_token) => bot_token,
            Err(_) => panic!("TELEGRAM_BOT_TOKEN environment variable does not exists!"),
        }
    )
});

pub static TELEGRAM_URL: Lazy<Url> = Lazy::new(|| {
    match Url::parse("https://api.telegram.org")
        .unwrap()
        .join(&TELEGRAM_BOT_TOKEN)
    {
        Ok(telegram_url) => telegram_url,
        Err(e) => panic!("TELEGRAM_URL with BOT_TOKEN build failed: {}", e),
    }
});
