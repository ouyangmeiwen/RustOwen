use deadpool_redis::Pool;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RedisClient {
    pub pool: Arc<Mutex<Pool>>, // 使用连接池
}
impl Clone for RedisClient {
    fn clone(&self) -> Self {
        // 使用 Arc::clone 来共享引用计数
        RedisClient {
            pool: Arc::clone(&self.pool),
        }
    }
}
