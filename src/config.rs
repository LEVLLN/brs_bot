use once_cell::sync::{Lazy, OnceCell};
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

pub static TELEGRAM_URL: OnceCell<Url> = OnceCell::new();

pub fn init_telegram_url(override_url: Option<String>) {
    #[cfg(test)]
    { TELEGRAM_URL.get_or_init(|| Url::parse(&override_url.unwrap()).unwrap()); }
    #[cfg(not(test))]
    {
        let mut auth_token = String::with_capacity(80);
        auth_token.push('/');
        auth_token.push_str(&std::env::var("TELEGRAM_BOT_ID").unwrap());
        auth_token.push(':');
        auth_token.push_str(&std::env::var("TELEGRAM_BOT_TOKEN").unwrap());
        auth_token.push('/');
        TELEGRAM_URL.get_or_init(|| {
            match Url::parse("https://api.telegram.org")
                .unwrap()
                .join(&auth_token) {
                Ok(telegram_url) => {
                    println!("{}", telegram_url.to_string().len());
                    telegram_url
                }
                Err(e) => panic!("TELEGRAM_URL with BOT_TOKEN build failed: {}", e),
            }
        });
    }
}
