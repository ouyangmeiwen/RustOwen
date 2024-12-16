use crate::models::config_model::Config;
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;
use std::sync::RwLock;
impl Config {
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

        Config {
            database_url,
            port,
            cors_allowed_origin,
            max_connections,
            log_level,
            redis_url,
            rabbitmq_uri,
            rabbitmq_exchange,
            rabbitmq_queue,
            rabbitmq_routing_key_send,
            rabbitmq_routing_key_revceived,
        }
    }
}
lazy_static! {
    // 使用 Mutex 包裹 Config，确保线程安全
    pub static ref STATIC_CONFIG: RwLock<Config> = RwLock::new(Config::new());
}
