use once_cell::sync::Lazy;
use url::Url;

pub static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    let database_str = std::env::var("DATABASE_URL").unwrap().to_string();
    let url = Url::parse(&database_str).expect("Not valid database url");
    assert_ne!(url.username(), "", "Username does not exists");
    assert_ne!(url.host(), None, "Host does not exists");
    assert_ne!(url.password(), None, "Password does not exists");
    assert_ne!(url.path(), "/", "Database name does not exists");
    assert_eq!(url.scheme(), "postgres", "Url need protocol: postgres");
    database_str
});
