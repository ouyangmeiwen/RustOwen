use crate::models::redisclient_model::RedisClient;
use actix_web::body::None;
use deadpool_redis::redis::aio::PubSub;
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::{Config, Connection};
use futures::future::ok;
use futures::stream::StreamExt;
use redis::AsyncCommands as RedisAsyncCommands; // Alias to avoid naming conflicts
use std::sync::Arc;
use tokio::sync::Mutex; // Import StreamExt to use `.next()`
impl RedisClient {
    pub async fn new(redis_url: &str) -> Result<Self, String> {
        let config = Config::from_url(redis_url);
        let pool = config
            .create_pool(None)
            .map_err(|e| format!("Failed to create Redis pool: {:?}", e))?;
        Ok(RedisClient {
            pool: Arc::new(Mutex::new(pool)),
        })
    }
    pub async fn get_connection(&self) -> Result<deadpool_redis::Connection, String> {
        let pool_lock = self.pool.lock().await;
        let conn = pool_lock
            .get()
            .await
            .map_err(|e| format!("Failed to get connection: {:?}", e))?;
        Ok(conn)
    }
    // 删除多个键
    pub async fn mdel(&self, keys: Vec<&str>) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;

        con.del(keys)
            .await
            .map_err(|e| format!("Failed to delete keys from Redis: {:?}", e))?;

        Ok(())
    }
    // 删除键并设置过期时间
    pub async fn del_and_expire(&self, key: &str, expire_seconds: usize) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;

        // 删除键
        con.del(key)
            .await
            .map_err(|e| format!("Failed to delete key from Redis: {:?}", e))?;

        // 设置过期时间
        con.expire(key, expire_seconds)
            .await
            .map_err(|e| format!("Failed to set expiration for key in Redis: {:?}", e))?;

        Ok(())
    }
    // 删除多个键并设置过期时间
    pub async fn mdel_and_expire(
        &self,
        keys: Vec<&str>,
        expire_seconds: usize,
    ) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;

        // 删除多个键
        con.del(keys.clone())
            .await
            .map_err(|e| format!("Failed to delete keys from Redis: {:?}", e))?;

        // 设置过期时间
        for key in keys.clone() {
            con.expire(key, expire_seconds)
                .await
                .map_err(|e| format!("Failed to set expiration for key in Redis: {:?}", e))?;
        }

        Ok(())
    }
    // 删除键
    pub async fn del(&self, key: &str) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;

        // 使用 AsyncCommands trait 执行 DEL 命令
        con.del(key)
            .await
            .map_err(|e| format!("Failed to delete key from Redis: {:?}", e))?;

        Ok(())
    }
    // 设置键的过期时间（秒）
    pub async fn expire(&self, key: &str, expire_seconds: usize) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;

        // 使用 AsyncCommands trait 执行 EXPIRE 命令
        con.expire(key, expire_seconds)
            .await
            .map_err(|e| format!("Failed to set expiration for key in Redis: {:?}", e))?;

        Ok(())
    }
    // 检查键是否存在
    pub async fn exists(&self, key: &str) -> Result<bool, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;

        // 使用 AsyncCommands trait 执行 EXISTS 命令
        con.exists(key)
            .await
            .map_err(|e| format!("Failed to check key existence in Redis: {:?}", e))
    }

    // 设置值
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

    // 获取值
    pub async fn get(&self, key: &str) -> Result<String, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.get(key)
            .await
            .map_err(|e| format!("Failed to get key from Redis: {:?}", e))
    }

    // 自增
    pub async fn incr(&self, key: &str) -> Result<i64, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.incr(key, 1)
            .await
            .map_err(|e| format!("Failed to increment key in Redis: {:?}", e))
    }

    // 自减
    pub async fn decr(&self, key: &str) -> Result<i64, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.decr(key, 1)
            .await
            .map_err(|e| format!("Failed to decrement key in Redis: {:?}", e))
    }

    // // 设置多个键值
    // pub async fn mset(&self, key_value_pairs: Vec<(&str, &str)>) -> Result<(), String> {
    //     let mut con = self
    //         .get_connection()
    //         .await
    //         .map_err(|e| format!("Redis connection error: {:?}", e))?;
    //     let pairs: Vec<(&str, &str)> = key_value_pairs.iter().map(|&(k, v)| (k, v)).collect();
    //     con.mset(pairs)
    //         .await
    //         .map_err(|e| format!("Failed to set multiple keys in Redis: {:?}", e))?;
    //     Ok(())
    // }

    // // 获取多个键值
    // pub async fn mget(&self, keys: Vec<&str>) -> Result<Vec<Option<String>>, String> {
    //     let mut con = self
    //         .get_connection()
    //         .await
    //         .map_err(|e| format!("Redis connection error: {:?}", e))?;
    //     con.mget(keys)
    //         .await
    //         .map_err(|e| format!("Failed to get multiple keys from Redis: {:?}", e))
    // }

    // // 仅在键不存在时设置值
    // pub async fn setnx(&self, key: &str, value: &str) -> Result<bool, String> {
    //     let mut con = self
    //         .get_connection()
    //         .await
    //         .map_err(|e| format!("Redis connection error: {:?}", e))?;
    //     con.setnx(key, value)
    //         .await
    //         .map_err(|e| format!("Failed to set key if not exists in Redis: {:?}", e))
    // }

    // 设置哈希表字段
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

    // 获取哈希表字段
    pub async fn hget(&self, hash_key: &str, field: &str) -> Result<String, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.hget(hash_key, field)
            .await
            .map_err(|e| format!("Failed to get field from hash: {:?}", e))
    }

    // 获取哈希表所有字段及值
    pub async fn hgetall(&self, hash_key: &str) -> Result<Vec<(String, String)>, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.hgetall(hash_key)
            .await
            .map_err(|e| format!("Failed to get all fields from hash: {:?}", e))
    }
    // 删除哈希表中的多个字段
    pub async fn hmdel(&self, hash_key: &str, fields: Vec<&str>) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;

        con.hdel(hash_key, fields)
            .await
            .map_err(|e| format!("Failed to delete fields from hash: {:?}", e))?;

        Ok(())
    }
    // 移除哈希表字段
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

    // 判断哈希表是否存在某个字段
    pub async fn hexists(&self, hash_key: &str, field: &str) -> Result<bool, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.hexists(hash_key, field)
            .await
            .map_err(|e| format!("Failed to check field existence in hash: {:?}", e))
    }

    // 从左侧插入值
    pub async fn lpush(&self, list_key: &str, values: Vec<String>) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.lpush(list_key, values)
            .await
            .map_err(|e| format!("Failed to push value to list (left): {:?}", e))?;
        Ok(())
    }

    // 从右侧插入值
    pub async fn rpush(&self, list_key: &str, values: Vec<String>) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.rpush(list_key, values)
            .await
            .map_err(|e| format!("Failed to push value to list (right): {:?}", e))?;
        Ok(())
    }

    // 获取列表中的所有元素
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

    // 获取列表的长度
    pub async fn llen(&self, list_key: &str) -> Result<i64, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.llen(list_key)
            .await
            .map_err(|e| format!("Failed to get list length: {:?}", e))
    }

    // 从左侧弹出值
    pub async fn lpop(&self, list_key: &str) -> Result<String, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.lpop(list_key, None)
            .await
            .map_err(|e| format!("Failed to pop value from list (left): {:?}", e))
    }

    // 从右侧弹出值
    pub async fn rpop(&self, list_key: &str) -> Result<String, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.rpop(list_key, None)
            .await
            .map_err(|e| format!("Failed to pop value from list (right): {:?}", e))
    }

    // 添加元素到集合
    pub async fn sadd(&self, set_key: &str, values: Vec<String>) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.sadd(set_key, values)
            .await
            .map_err(|e| format!("Failed to add value to set: {:?}", e))?;
        Ok(())
    }

    // 获取集合的成员
    pub async fn smembers(&self, set_key: &str) -> Result<Vec<String>, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.smembers(set_key)
            .await
            .map_err(|e| format!("Failed to get members of set: {:?}", e))
    }

    // 从集合中移除元素
    pub async fn srem(&self, set_key: &str, values: Vec<String>) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.srem(set_key, values)
            .await
            .map_err(|e| format!("Failed to remove value from set: {:?}", e))?;
        Ok(())
    }

    // 判断元素是否在集合中
    pub async fn sismember(&self, set_key: &str, value: &str) -> Result<bool, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.sismember(set_key, value)
            .await
            .map_err(|e| format!("Failed to check member in set: {:?}", e))
    }

    // 获取集合的成员数
    pub async fn scard(&self, set_key: &str) -> Result<i64, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;
        con.scard(set_key)
            .await
            .map_err(|e| format!("Failed to get set cardinality: {:?}", e))
    }
    // 获取键的过期时间（TTL）
    pub async fn ttl(&self, key: &str) -> Result<i64, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;

        con.ttl(key)
            .await
            .map_err(|e| format!("Failed to get TTL for key in Redis: {:?}", e))
    }
    // 向有序集合中添加元素
    pub async fn zadd(&self, zset_key: &str, score: f64, value: &str) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;

        con.zadd(zset_key, value, score)
            .await
            .map_err(|e| format!("Failed to add element to sorted set: {:?}", e))?;

        Ok(())
    }
    // 获取有序集合中的元素（按分数排序）
    pub async fn zrange(
        &self,
        zset_key: &str,
        start: isize,
        stop: isize,
    ) -> Result<Vec<String>, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;

        con.zrange(zset_key, start, stop)
            .await
            .map_err(|e| format!("Failed to get elements from sorted set: {:?}", e))
    }
    // 获取有序集合元素的分数
    pub async fn zscore(&self, zset_key: &str, value: &str) -> Result<Option<f64>, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;

        con.zscore(zset_key, value)
            .await
            .map_err(|e| format!("Failed to get score for element from sorted set: {:?}", e))
    }
    // 发布消息到频道
    pub async fn publish(&self, channel: &str, message: &str) -> Result<(), String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;

        con.publish(channel, message)
            .await
            .map_err(|e| format!("Failed to publish message to Redis channel: {:?}", e))?;

        Ok(())
    }
    // 订阅频道
    pub async fn subscribe(&self, channels: Vec<&str>) -> Result<(), String> {
        Ok(())
        // let mut con = self
        //     .get_connection()
        //     .await
        //     .map_err(|e| format!("Redis connection error: {:?}", e))?;

        // // Create a PubSub connection for subscribing
        // let mut pubsub: PubSub = con.into_pubsub();

        // // Subscribe to the provided channels
        // pubsub
        //     .subscribe(channels)
        //     .await
        //     .map_err(|e| format!("Failed to subscribe to channels: {:?}", e))?;

        // // Listen for messages from the subscribed channels
        // loop {
        //     // Use `.next()` to get the next message from the stream
        //     if let Some(msg) = pubsub.next().await {
        //         // Process the message (e.g., get the payload)
        //         match msg.get_payload::<String>().await {
        //             Ok(payload) => {
        //                 // Print the received message
        //                 println!("Received message: {}", payload);
        //                 // You can implement additional logic here to handle the message
        //             }
        //             Err(e) => {
        //                 // Handle error when reading payload
        //                 println!("Error reading message payload: {:?}", e);
        //             }
        //         }
        //     }
        // }
    }

    // 获取分布式锁
    pub async fn lock(
        &self,
        lock_key: &str,
        lock_value: &str,
        ttl_seconds: usize,
    ) -> Result<bool, String> {
        let mut con = self
            .get_connection()
            .await
            .map_err(|e| format!("Redis connection error: {:?}", e))?;

        // 使用 SETNX 设置锁，只有在键不存在时才成功
        let result: i32 = con
            .set_nx(lock_key, lock_value)
            .await
            .map_err(|e| format!("Failed to acquire lock: {:?}", e))?;

        if result == 1 {
            // 锁成功，设置过期时间
            con.expire(lock_key, ttl_seconds)
                .await
                .map_err(|e| format!("Failed to set lock expiration: {:?}", e))?;
            Ok(true)
        } else {
            // 锁失败
            Ok(false)
        }
    }

    // 释放锁
    pub async fn unlock(&self, lock_key: &str, lock_value: &str) -> Result<(), String> {
        Ok(())
        //         // Get a connection from the pool
        //         let mut con = self
        //             .get_connection()
        //             .await
        //             .map_err(|e| format!("Redis connection error: {:?}", e))?;

        //         // Lua script to ensure only the holder of the lock can release it
        //         let script = r#"
        //     if redis.call('get', KEYS[1]) == ARGV[1] then
        //         return redis.call('del', KEYS[1])
        //     else
        //         return 0
        //     end
        // "#;

        //         // Use the `cmd` method to execute the Lua script with `eval`
        //         let result: i32 = con
        //             .cmd("eval")
        //             .arg(script)
        //             .arg(1) // KEYS length (number of keys)
        //             .arg(lock_key) // The lock key
        //             .arg(lock_value) // The lock value
        //             .query_async(&mut con)
        //             .await
        //             .map_err(|e| format!("Failed to release lock: {:?}", e))?;

        //         if result == 1 {
        //             Ok(())
        //         } else {
        //             Err("Failed to release lock: Lock is held by another client".into())
        //         }
    }
}
