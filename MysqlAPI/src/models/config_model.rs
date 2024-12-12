pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub cors_allowed_origin: String,
    pub max_connections: u32,
    pub log_level: String,
    pub redis_url: String,

    pub rabbitmq_uri: String,
    pub rabbitmq_exchange: String,
    pub rabbitmq_queue: String,
    pub rabbitmq_routing_key_send: String,
    pub rabbitmq_routing_key_revceived: String,
}
