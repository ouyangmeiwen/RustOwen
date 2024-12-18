use crate::models::config_model::Config;
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde_yaml;
use std::env;
use std::fs;
use std::string::String;
use std::sync::RwLock; // Ensure to add serde_yaml as a dependency in Cargo.toml
impl Config {
    pub fn new_yarm() -> Config {
        // Read the file contents as raw bytes (Vec<u8>)
        let contents: Vec<u8> = fs::read("config.yaml").expect("Failed to read file");
        // Convert the bytes into a UTF-8 string
        let string_contents =
            String::from_utf8(contents).expect("Failed to convert bytes to string");
        // Parse the string contents (YAML) into a Config struct
        let config: Config = serde_yaml::from_str(&string_contents).expect("Failed to parse YAML");
        config
    }
    pub fn new() -> Config {
        dotenv().ok(); // 加载 .env 文件

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

        let port = env::var("PORT")
            .expect("PORT must be set in .env file")
            .parse()
            .expect("PORT must be a valid u16");

        let cors_allowed_origin =
            env::var("CORS_ALLOWED_ORIGIN").expect("CORS_ALLOWED_ORIGIN must be set in .env file");

        let max_connections = env::var("MAX_CONNECTIONS")
            .expect("MAX_CONNECTIONS must be set in .env file")
            .parse()
            .expect("MAX_CONNECTIONS must be a valid u32");

        let log_level = env::var("LOG_LEVEL").expect("LOG_LEVEL must be set in .env file");
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set in .env file");
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set in .env file");

        let rabbitmq_uri = env::var("RABBITMQ_URI").expect("RABBITMQ_URI must be set in .env file");
        let rabbitmq_exchange =
            env::var("RABBITMQ_EXCHANGE").expect("RABBITMQ_EXCHANGE must be set in .env file");
        let rabbitmq_queue =
            env::var("RABBITMQ_QUEUE").expect("RABBITMQ_QUEUE must be set in .env file");
        let rabbitmq_routing_key_send = env::var("RABBITMQ_ROUTING_KEY_SEND")
            .expect("RABBITMQ_ROUTING_KEY_SEND must be set in .env file");
        let rabbitmq_routing_key_revceived = env::var("RABBITMQ_ROUTING_KEY_RECEIVED")
            .expect("RABBITMQ_ROUTING_KEY_RECEIVED must be set in .env file");

        let limit_per_second_default = env::var("LIMIT_PER_SECOND_DEFAULT")
            .expect("LIMIT_PER_SECOND_DEFAULT must be set in .env file")
            .parse()
            .expect("LIMIT_PER_SECOND_DEFAULT must be a valid u64");

        let time_window_secs_default = env::var("TIME_WINDOW_SECS_DEFAULT")
            .expect("TIME_WINDOW_SECS_DEFAULT must be set in .env file")
            .parse()
            .expect("TIME_WINDOW_SECS_DEFAULT must be a valid u64");

        let limit_ip = env::var("LIMIT_IP")
            .expect("LIMIT_IP must be set in .env file")
            .parse()
            .expect("LIMIT_IP must be a valid u64");

        Config {
            database_url,
            port,
            cors_allowed_origin,
            max_connections,
            log_level,
            secret_key,
            redis_url,
            rabbitmq_uri,
            rabbitmq_exchange,
            rabbitmq_queue,
            rabbitmq_routing_key_send,
            rabbitmq_routing_key_revceived,
            limit_per_second_default,
            time_window_secs_default,
            limit_ip,
        }
    }
}
lazy_static! {
    // 使用 Mutex 包裹 Config，确保线程安全
    pub static ref STATIC_CONFIG: RwLock<Config> = RwLock::new(Config::new());
}
