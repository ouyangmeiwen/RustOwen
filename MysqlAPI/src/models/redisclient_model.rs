use once_cell::sync::Lazy; // 导入 Lazy
use redis::aio::Connection;
use redis::{AsyncCommands, Client, RedisResult};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RedisClient {
    pub client: Arc<Mutex<Client>>,
    pub pool: Arc<Mutex<Option<redis::aio::Connection>>>, // Option 用来存储连接池
}
impl Clone for RedisClient {
    fn clone(&self) -> Self {
        // 使用 Arc::clone 来共享引用计数
        RedisClient {
            client: Arc::clone(&self.client),
            pool: Arc::clone(&self.pool),
        }
    }
}
