use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub build_address: String,
}

pub fn config() -> Config {
    dotenv().ok();
    Config {
        build_address: env::var("BUILD_ADDRESS").expect("DATABASE_URL not set!"),
    }
}
