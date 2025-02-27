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
    fn get_env_var<T: std::str::FromStr>(key: &str, default: T) -> T {
        match env::var(key) {
            Ok(value) => value
                .parse::<T>()
                .unwrap_or_else(|_| panic!("Failed to parse {} from environment", key)),
            // Err(_) => default.unwrap_or_else(|| panic!("{} must be set in .env file", key)),
            Err(_) => default,
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
        let contents = fs::read("config.yaml").expect("Failed to read config.yaml");
        let string_contents =
            String::from_utf8(contents).expect("Failed to convert bytes to string");
        let config: Config = serde_yaml::from_str(&string_contents).expect("Failed to parse YAML");
        config
        //dotenv().ok(); // 加载 .env 文件
        // Config {
        //     database_url: Config::get_env_var(
        //         "DATABASE_URL",
        //         String::from("mysql://root:abc@123@192.168.229.130:3306/invengodbv41"),
        //     ),
        //     port: Config::get_env_var("PORT", 7788),
        //     cors_allowed_origin: Config::get_env_var(
        //         "CORS_ALLOWED_ORIGIN",
        //         String::from("http://192.168.229.130:3000"),
        //     ),
        //     max_connections: Config::get_env_var("MAX_CONNECTIONS", 10),
        //     log_level: Config::get_env_var("LOG_LEVEL", String::from("actix_web=info")),
        //     secret_key: Config::get_env_var("SECRET_KEY", String::from("123123123123")),
        //     redis_url: Config::get_env_var("REDIS_URL", String::from("redis://:002161@192.168.229.130:6379/")),
        //     rabbitmq_uri: Config::get_env_var("RABBITMQ_URI", String::new()), // 如果没有该值，返回空字符串
        //     rabbitmq_exchange: Config::get_env_var(
        //         "RABBITMQ_EXCHANGE",
        //         String::from("exchange_topic"),
        //     ),
        //     rabbitmq_queue: Config::get_env_var("RABBITMQ_QUEUE", String::from("queue_task")),
        //     rabbitmq_routing_key_send: Config::get_env_var(
        //         "RABBITMQ_ROUTING_KEY_SEND",
        //         String::from("routing_key.key.task.sendmsg"),
        //     ),
        //     rabbitmq_routing_key_revceived: Config::get_env_var(
        //         "RABBITMQ_ROUTING_KEY_RECEIVED",
        //         String::from("routing_key.key.task.*"),
        //     ),
        //     limit_per_second_default: Config::get_env_var("LIMIT_PER_SECOND_DEFAULT", 50),
        //     time_window_secs_default: Config::get_env_var("TIME_WINDOW_SECS_DEFAULT", 10),
        //     limit_ip: Config::get_env_var("LIMIT_IP", false),
        //     sso: Config::get_env_var("SSO", true),
        // }
    }
}

lazy_static! {
    // 使用 RwLock 包裹 Config，确保线程安全
    pub static ref STATIC_CONFIG: RwLock<Config> = RwLock::new(Config::new());
}
