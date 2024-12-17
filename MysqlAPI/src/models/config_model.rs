/// 应用程序配置结构体
#[derive(Clone)]
pub struct Config {
    /// 数据库连接 URL
    pub database_url: String,

    /// 服务器监听端口
    pub port: u16,

    /// 允许跨域访问的源地址
    pub cors_allowed_origin: String,

    /// 数据库连接池的最大连接数
    pub max_connections: u32,

    /// actix-web 日志级别
    pub log_level: String,

    /// 用于 JWT 认证的密钥
    pub secret_key: String,

    /// Redis 连接 URL
    pub redis_url: String,

    /// RabbitMQ 连接 URI
    pub rabbitmq_uri: String,

    /// RabbitMQ 交换机名称
    pub rabbitmq_exchange: String,

    /// RabbitMQ 消息队列名称
    pub rabbitmq_queue: String,

    /// RabbitMQ 消息发送者的路由键
    pub rabbitmq_routing_key_send: String,

    /// RabbitMQ 消息接收者的路由键
    pub rabbitmq_routing_key_revceived: String,

    /// 默认单位时间内的访问限制数量
    pub limit_per_second_default: u64,

    /// 访问限制的单位时间（秒）
    pub time_window_secs_default: u64,

    /// 是否启用基于 IP 的访问限制
    pub limit_ip: bool,
}
