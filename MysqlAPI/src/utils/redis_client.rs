use crate::configs::envconfig::Config;
use once_cell::sync::Lazy; // 导入 Lazy
use redis::aio::Connection;
use redis::{AsyncCommands, Client, RedisResult};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RedisClient {
    client: Arc<Mutex<Client>>,
    pool: Arc<Mutex<Option<redis::aio::Connection>>>, // Option 用来存储连接池
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
// // 创建全局 RedisClient 单例实例
// pub static REDIS_CLIENT: Lazy<Arc<Mutex<Option<RedisClient>>>> = Lazy::new(|| {
//     Arc::new(Mutex::new(None)) // 初始为空
// });

impl RedisClient {
    // 创建一个新的 RedisClient 实例
    pub async fn new(redis_url: &str) -> Result<Self, String> {
        let client = Client::open(redis_url)
            .map_err(|e| format!("Failed to create Redis client: {:?}", e))?;
        let pool = Arc::new(Mutex::new(None)); // 初始为空，可以根据需求初始化连接池
        Ok(RedisClient {
            client: Arc::new(Mutex::new(client)),
            pool,
        })
    }
    // // 使用默认配置创建 RedisClient 实例
    // pub async fn default() -> Result<Self, String> {
    //     let config: Config = Config::new();
    //     let redis_url = &config.redis_url;

    //     // 获取 REDIS_CLIENT 的锁
    //     let mut redis_client_guard = REDIS_CLIENT.lock().await;

    //     // 如果 REDIS_CLIENT 已经存在，复用已有的 RedisClient
    //     if let Some(ref client) = *redis_client_guard {
    //         // 如果连接池为空，创建连接
    //         return Ok(client.clone());
    //     }
    //     // 如果 REDIS_CLIENT 不存在，创建新的实例
    //     let client = Client::open(redis_url.to_string())
    //         .map_err(|e| format!("Failed to create Redis client: {:?}", e))?;

    //     let pool = Arc::new(Mutex::new(None)); // 初始为空
    //     let new_client = RedisClient {
    //         client: Arc::new(Mutex::new(client)),
    //         pool,
    //     };
    //     // 更新 REDIS_CLIENT 单例，保存新的 RedisClient 实例
    //     *redis_client_guard = Some(new_client.clone()); // 使用 Arc::clone 来共享实例
    //     Ok(new_client)
    // }

    // 获取 Redis 连接池中的连接
    pub async fn get_connection(&self) -> RedisResult<redis::aio::Connection> {
        let mut pool_lock = self.pool.lock().await;

        // 如果池中没有连接，创建一个新的连接并存储在池中
        if pool_lock.is_none() {
            let client_lock = self.client.lock().await;
            let con = client_lock.get_async_connection().await?;
            *pool_lock = Some(con);
        }

        // 返回连接池中的连接
        pool_lock.take().ok_or_else(|| {
            redis::RedisError::from((redis::ErrorKind::IoError, "Connection pool is empty"))
        })
    }

    // 关闭连接池并释放连接
    pub async fn close_connection(&self) {
        let mut pool_lock = self.pool.lock().await;
        *pool_lock = None; // 将连接池设置为空
    }
    // 设置 Redis 键值对
    pub async fn set(&self, key: &str, value: &str) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.set(key, value)
            .await
            .map_err(|e| format!("Failed to set key in Redis: {:?}", e))?;
        Ok(())
    }

    // 获取 Redis 键值
    pub async fn get(&self, key: &str) -> Result<String, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.get(key)
            .await
            .map_err(|e| format!("Failed to get key from Redis: {:?}", e))
    }

    // 删除 Redis 键
    pub async fn del(&self, key: &str) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.del(key)
            .await
            .map_err(|e| format!("Failed to delete key from Redis: {:?}", e))?;
        Ok(())
    }

    // 设置 Redis 键的过期时间
    pub async fn expire(&self, key: &str, expire_seconds: i64) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.expire(key, expire_seconds)
            .await
            .map_err(|e| format!("Failed to set expiration for key in Redis: {:?}", e))?;
        Ok(())
    }

    // 检查 Redis 中是否存在某个键
    pub async fn exists(&self, key: &str) -> Result<bool, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.exists(key)
            .await
            .map_err(|e| format!("Failed to check key existence in Redis: {:?}", e))
    }

    // 设置 Redis 键值对并设置过期时间
    pub async fn set_ex(&self, key: &str, value: &str, expire_seconds: u64) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.set_ex(key, value, expire_seconds)
            .await
            .map_err(|e| format!("Failed to set key with expiration in Redis: {:?}", e))?;
        Ok(())
    }

    // 获取 Redis 键的过期时间
    pub async fn ttl(&self, key: &str) -> Result<i64, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.ttl(key)
            .await
            .map_err(|e| format!("Failed to get TTL for key in Redis: {:?}", e))
    }

    // 哈希表操作
    pub async fn hset(&self, hash_key: &str, field: &str, value: &str) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.hset(hash_key, field, value)
            .await
            .map_err(|e| format!("Failed to set field in hash: {:?}", e))?;
        Ok(())
    }

    // 获取 Redis 哈希表字段
    pub async fn hget(&self, hash_key: &str, field: &str) -> Result<String, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.hget(hash_key, field)
            .await
            .map_err(|e| format!("Failed to get field from hash: {:?}", e))
    }

    // 删除 Redis 哈希表字段
    pub async fn hdel(&self, hash_key: &str, field: &str) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.hdel(hash_key, field)
            .await
            .map_err(|e| format!("Failed to delete field from hash: {:?}", e))?;
        Ok(())
    }

    // 获取 Redis 哈希表所有字段和值
    pub async fn hgetall(&self, hash_key: &str) -> Result<Vec<(String, String)>, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.hgetall(hash_key)
            .await
            .map_err(|e| format!("Failed to get all fields from hash: {:?}", e))
    }

    // 设置 Redis 列表
    pub async fn lpush(&self, list_key: &str, values: Vec<String>) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        for value in values {
            con.lpush(list_key, value)
                .await
                .map_err(|e| format!("Failed to push value to list: {:?}", e))?;
        }
        Ok(())
    }

    // 获取 Redis 列表中的所有元素
    pub async fn lrange(
        &self,
        list_key: &str,
        start: isize,
        stop: isize,
    ) -> Result<Vec<String>, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.lrange(list_key, start, stop)
            .await
            .map_err(|e| format!("Failed to get values from list: {:?}", e))
    }

    // 从 Redis 列表中移除并返回左侧的元素
    pub async fn lpop(&self, list_key: &str) -> Result<String, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.lpop(list_key, None)
            .await
            .map_err(|e| format!("Failed to pop value from list: {:?}", e))
    }

    // 从 Redis 列表中移除并返回右侧的元素
    pub async fn rpop(&self, list_key: &str) -> Result<String, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.rpop(list_key, None)
            .await
            .map_err(|e| format!("Failed to pop value from list: {:?}", e))
    }

    // 设置 Redis 集合
    pub async fn sadd(&self, set_key: &str, values: Vec<String>) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        for value in values {
            con.sadd(set_key, value)
                .await
                .map_err(|e| format!("Failed to add value to set: {:?}", e))?;
        }
        Ok(())
    }

    // 移除 Redis 集合中的成员
    pub async fn srem(&self, set_key: &str, values: Vec<String>) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        for value in values {
            con.srem(set_key, value)
                .await
                .map_err(|e| format!("Failed to remove value from set: {:?}", e))?;
        }
        Ok(())
    }

    // 获取 Redis 集合中的所有成员
    pub async fn smembers(&self, set_key: &str) -> Result<Vec<String>, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.smembers(set_key)
            .await
            .map_err(|e| format!("Failed to get values from set: {:?}", e))
    }
}
