use crate::models::config_model::Config;
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde_yaml;
use std::env;
use std::fs;
use std::string::String;
use std::sync::RwLock;

impl Config {
    // 辅助函数：从环境变量中获取并解析值
    fn get_env_var<T: std::str::FromStr>(key: &str, default: Option<T>) -> T {
        match env::var(key) {
            Ok(value) => value
                .parse::<T>()
                .unwrap_or_else(|_| panic!("Failed to parse {} from environment", key)),
            Err(_) => default.unwrap_or_else(|| panic!("{} must be set in .env file", key)),
        }
    }

    pub fn new_yarn() -> Config {
        // 读取 YAML 配置文件并解析
        let contents = fs::read("config.yaml").expect("Failed to read config.yaml");
        let string_contents =
            String::from_utf8(contents).expect("Failed to convert bytes to string");
        let config: Config = serde_yaml::from_str(&string_contents).expect("Failed to parse YAML");
        config
    }

    pub fn new() -> Config {
        dotenv().ok(); // 加载 .env 文件

        Config {
            database_url: Config::get_env_var("DATABASE_URL", None),
            port: Config::get_env_var("PORT", None),
            cors_allowed_origin: Config::get_env_var("CORS_ALLOWED_ORIGIN", None),
            max_connections: Config::get_env_var("MAX_CONNECTIONS", None),
            log_level: Config::get_env_var("LOG_LEVEL", None),
            secret_key: Config::get_env_var("SECRET_KEY", None),
            redis_url: Config::get_env_var("REDIS_URL", None),
            rabbitmq_uri: Config::get_env_var("RABBITMQ_URI", None),
            rabbitmq_exchange: Config::get_env_var("RABBITMQ_EXCHANGE", None),
            rabbitmq_queue: Config::get_env_var("RABBITMQ_QUEUE", None),
            rabbitmq_routing_key_send: Config::get_env_var("RABBITMQ_ROUTING_KEY_SEND", None),
            rabbitmq_routing_key_revceived: Config::get_env_var(
                "RABBITMQ_ROUTING_KEY_RECEIVED",
                None,
            ),
            limit_per_second_default: Config::get_env_var("LIMIT_PER_SECOND_DEFAULT", None),
            time_window_secs_default: Config::get_env_var("TIME_WINDOW_SECS_DEFAULT", None),
            limit_ip: Config::get_env_var("LIMIT_IP", None),
        }
    }
}

lazy_static! {
    // 使用 RwLock 包裹 Config，确保线程安全
    pub static ref STATIC_CONFIG: RwLock<Config> = RwLock::new(Config::new());
}
