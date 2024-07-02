#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| String::from("test_db"));
        Config { database_url }
    }
}
