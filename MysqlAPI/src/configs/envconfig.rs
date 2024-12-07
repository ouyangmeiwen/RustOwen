// src/configs/envconfig.rs

use std::env;
use dotenv::dotenv;

pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub cors_allowed_origin: String,
    pub max_connections: u32,
    pub log_level: String,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok(); // 加载 .env 文件

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env file");

        let port = env::var("PORT")
            .expect("PORT must be set in .env file")
            .parse()
            .expect("PORT must be a valid u16");

        let cors_allowed_origin = env::var("CORS_ALLOWED_ORIGIN")
            .expect("CORS_ALLOWED_ORIGIN must be set in .env file");

        let max_connections = env::var("MAX_CONNECTIONS")
            .expect("MAX_CONNECTIONS must be set in .env file")
            .parse()
            .expect("MAX_CONNECTIONS must be a valid u32");

        let log_level = env::var("LOG_LEVEL")
            .expect("LOG_LEVEL must be set in .env file");

        Config {
            database_url,
            port,
            cors_allowed_origin,
            max_connections,
            log_level,
        }
    }
}
