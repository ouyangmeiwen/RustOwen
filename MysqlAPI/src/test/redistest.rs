use redis::{Client, Commands, RedisResult};

// 创建一个 Redis 客户端并连接到 Redis 服务器
fn create_connection() -> RedisResult<redis::Connection> {
    //let client = redis::Client::open("redis://:002161@127.0.0.1:6379/")?;
    let client = redis::Client::open("redis://127.0.0.1:6379/")?;
    let con = client.get_connection()?;
    Ok(con)
}

// 设置字符串键值
fn set_string(con: &mut redis::Connection, key: &str, value: &str) -> RedisResult<()> {
    con.set(key, value)?;
    Ok(())
}

// 获取字符串键值
fn get_string(con: &mut redis::Connection, key: &str) -> RedisResult<String> {
    let value: String = con.get(key)?;
    Ok(value)
}

// 设置哈希值
fn set_hash(
    con: &mut redis::Connection,
    hash_key: &str,
    field: &str,
    value: i32,
) -> RedisResult<()> {
    con.hset(hash_key, field, value)?;
    Ok(())
}

// 获取哈希值
fn get_hash(con: &mut redis::Connection, hash_key: &str, field: &str) -> RedisResult<i32> {
    let value: i32 = con.hget(hash_key, field)?;
    Ok(value)
}

// 设置列表
fn set_list(con: &mut redis::Connection, list_key: &str, values: Vec<i32>) -> RedisResult<()> {
    for value in values {
        con.rpush(list_key, value)?;
    }
    Ok(())
}

// 获取列表
fn get_list(con: &mut redis::Connection, list_key: &str) -> RedisResult<Vec<i32>> {
    let list: Vec<i32> = con.lrange(list_key, 0, -1)?;
    Ok(list)
}

// 设置集合
fn set_set(con: &mut redis::Connection, set_key: &str, values: Vec<i32>) -> RedisResult<()> {
    for value in values {
        con.sadd(set_key, value)?;
    }
    Ok(())
}

// 获取集合
fn get_set(con: &mut redis::Connection, set_key: &str) -> RedisResult<Vec<i32>> {
    let set: Vec<i32> = con.smembers(set_key)?;
    Ok(set)
}

// 设置有序集合
fn set_zset(
    con: &mut redis::Connection,
    zset_key: &str,
    values: Vec<(f64, &str)>,
) -> RedisResult<()> {
    for (score, member) in values {
        // 确保分数是 f64 类型，若传递的是整数，可以显式转换
        println!("Adding to ZSet: Score = {}, Member = {}", score, member); // 打印分数和值
        let score = score as f64; // 将整数显式转换为 f64
        con.zadd(zset_key, score, member)?;
    }
    Ok(())
}

fn get_zset(con: &mut redis::Connection, zset_key: &str) -> RedisResult<Vec<(f64, String)>> {
    let result: Vec<(f64, String)> = con.zrange_withscores(zset_key, 0, -1)?;
    Ok(result)
}

// 删除键
fn delete_key(con: &mut redis::Connection, key: &str) -> RedisResult<()> {
    con.del(key)?;
    Ok(())
}

// 检查键是否存在
fn check_key_exists(con: &mut redis::Connection, key: &str) -> RedisResult<bool> {
    let exists: bool = con.exists(key)?;
    Ok(exists)
}

// 设置键的过期时间（秒）
fn set_key_expiration(con: &mut redis::Connection, key: &str, seconds: i64) -> RedisResult<()> {
    con.expire(key, seconds)?;
    Ok(())
}

// 示例用法
pub fn test_redis() -> RedisResult<()> {
    let mut con = create_connection()?;

    // 操作示例
    set_string(&mut con, "my_string_key", "Hello, Redis!")?;
    let value = get_string(&mut con, "my_string_key")?;
    println!("my_string_key = {}", value);

    // 哈希示例
    set_hash(&mut con, "my_hash", "field1", 100)?;
    let hash_value = get_hash(&mut con, "my_hash", "field1")?;
    println!("my_hash[field1] = {}", hash_value);

    // 列表示例
    set_list(&mut con, "my_list", vec![10, 20, 30])?;
    let list = get_list(&mut con, "my_list")?;
    println!("my_list = {:?}", list);

    // 集合示例
    set_set(&mut con, "my_set", vec![1, 2, 3])?;
    let set = get_set(&mut con, "my_set")?;
    println!("my_set = {:?}", set);

    // // 执行有序集合操作并捕获错误
    // set_zset(&mut con, "my_zset", vec![(1.0, "one"), (2.0, "two")]).unwrap_or_else(|err| {
    //     eprintln!("Error setting ZSet: {}", err);
    //     std::process::exit(1);
    // });

    // let zset = get_zset(&mut con, "my_zset").unwrap_or_else(|err| {
    //     eprintln!("Error getting ZSet: {}", err);
    //     std::process::exit(1);
    // });

    //println!("my_zset = {:?}", zset);

    // 删除键示例
    delete_key(&mut con, "my_string_key")?;

    // 检查键是否存在
    let exists = check_key_exists(&mut con, "my_string_key")?;
    println!("Does my_string_key exist? {}", exists);

    // 设置键的过期时间
    set_key_expiration(&mut con, "my_hash", 10)?;

    Ok(())
}
