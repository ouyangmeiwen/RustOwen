use super::redisclient_model::RedisClient;
use crate::models::rabbitmq_model::RabbitMQ;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool}; // 使用 MySql // 引用 handler 模块
use std::sync::Arc;
use std::sync::Mutex;
pub struct AppState {
    pub db: Pool<MySql>,                 // 将 Pool<Postgres> 改为 Pool<MySql>
    pub redis_client: RedisClient,       // Redis 客户端
    pub rabbitmq: Option<Arc<RabbitMQ>>, // 修改为 Arc<RabbitMQ>
}

impl AppState {
    pub fn new(
        db: Pool<MySql>,
        redis_client: RedisClient,
        rabbitmq: Option<Arc<RabbitMQ>>,
    ) -> Self {
        AppState {
            db,
            redis_client,
            rabbitmq,
        }
    }
}
