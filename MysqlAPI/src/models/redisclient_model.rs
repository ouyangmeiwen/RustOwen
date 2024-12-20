use deadpool_redis::Pool;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RedisClient {
    pub pool: Pool, // 直接使用连接池 这个pool 是线性安全的
}

impl Clone for RedisClient {
    fn clone(&self) -> Self {
        RedisClient {
            pool: self.pool.clone(),
        }
    }
}
