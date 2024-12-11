use super::redisclient_model::RedisClient;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool}; // 使用 MySql // 引用 handler 模块

pub struct AppState {
    pub db: Pool<MySql>,           // 将 Pool<Postgres> 改为 Pool<MySql>
    pub redis_client: RedisClient, // Redis 客户端
}

impl AppState {
    pub fn new(db: Pool<MySql>, redis_client: RedisClient) -> Self {
        AppState { db, redis_client }
    }
}
