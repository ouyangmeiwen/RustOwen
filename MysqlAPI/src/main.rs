mod configs;
pub mod handlers;
mod middlewares;
pub mod models;
pub mod schemas;
pub mod test;
mod utils;
use crate::handlers::router_handler;
use crate::handlers::websocket_handler::websocket_register;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use configs::envconfig::STATIC_CONFIG;
use dotenv::dotenv;
use log::info;
use middlewares::auth_middleware::Auth;
use middlewares::limit_middleware::RateLimitMiddleware;
use models::config_model::Config;
use utils::websockethelper::WebSocketHelper;

use crate::models::static_model::*;
//use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use middlewares::jwt::JwtMiddleware;
use models::appstate_model::AppState;
use models::rabbitmq_model::RabbitMQ;
use models::redisclient_model::RedisClient;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool}; // 使用 MySql // 引用 handler 模块
use std::sync::Arc;
use std::sync::Mutex;
use test::a_testdemo;
use tokio::sync::mpsc; // 异步版的 mpsc
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    a_testdemo::Test();
    dotenv().ok();
    let config: Config = STATIC_CONFIG.read().unwrap().clone(); //智能指针
    let log_level = config.log_level.clone(); // 获取日志级别配置
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", log_level);
    }
    env_logger::init();
    let database_url = &config.database_url;
    let port = config.port;
    let max_connections = config.max_connections;

    let pool = match MySqlPoolOptions::new() // 使用 MySqlPoolOptions
        .max_connections(max_connections)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("start http service:http://127.0.0.1:{}", port); // 使用 format! 或 {} 来插入变量
    let redis_client = RedisClient::new(&config.redis_url)
        .await
        .expect("Failed to initialize Redis client");

    let mut rabbitmq_use: Option<Arc<RabbitMQ>> = None;
    let rabbitmq_uri = &config.rabbitmq_uri;
    if !rabbitmq_uri.clone().is_empty() {
        // 这里初始化静态变量，将配置中的值存储到全局变量
        // 锁住 Mutex 来修改静态变量
        let mut received_key = RABBITMQ_ROUTING_EXCHANGE.write().unwrap();
        *received_key = config.rabbitmq_exchange.clone();

        // 创建 RabbitMQ 实例，并打印错误信息
        let rabbitmq_new: Arc<RabbitMQ> = match RabbitMQ::new(rabbitmq_uri).await {
            Ok(rmq) => Arc::new(rmq),
            Err(e) => {
                eprintln!("Failed to connect to RabbitMQ: {}", e);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e)); // 返回错误
            }
        };
        rabbitmq_use = Some(rabbitmq_new.clone());
        // 设置消息通道，用于与后台任务通信
        let (tx, mut rx) = mpsc::channel::<String>(100);
        // 启动后台任务，开始消费消息
        tokio::spawn({
            let rabbitmq_consumer = rabbitmq_use.as_ref().unwrap().clone(); // 克隆 Arc
            async move {
                rabbitmq_consumer
                    .consume(
                        &config.rabbitmq_exchange,
                        &config.rabbitmq_queue,
                        &config.rabbitmq_routing_key_revceived,
                        tx,
                    )
                    .await
                    .unwrap();
            }
        });
    }
    // Set the rate limit (e.g., 10 requests per second)
    let rate_limit_middleware: RateLimitMiddleware = RateLimitMiddleware::new(
        config.limit_per_second_default,
        config.time_window_secs_default,
    );
    println!(
        "start api limit:{}/{}",
        config.limit_per_second_default, config.time_window_secs_default
    );
    HttpServer::new(move || {
        let cors: Cors = Cors::default()
            .allowed_origin(&config.cors_allowed_origin) // 直接使用 config
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                redis_client: redis_client.clone(),
                rabbitmq: rabbitmq_use.clone(),
            }))
            .route("/ws/{client_id}", web::get().to(websocket_register)) // WebSocket route
            .configure(router_handler::config)
            .wrap(cors)
            .wrap(rate_limit_middleware.clone()) // Inject the middleware into the app
            .wrap(Logger::default())
            .wrap(Auth {}) // Wrap the Auth middleware here
                           //.wrap(JwtMiddleware) // 应用 JWT 中间件
                           // .wrap(
                           //     RateLimiter::new(MemoryStoreActor::from(store.clone()).start())
                           //         .with_interval(Duration::from_secs(1))
                           //         .with_max_requests(1),
                           // )
    })
    .bind(("0.0.0.0", port))?
    .shutdown_timeout(30) // 设置优雅关闭的超时，单位是秒
    .run()
    .await
}
