// src/config/mod.rs

use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Config {
    pub db_url: String,
    pub secret_key: String,  // 用于 Token 校验
}

impl Config {
    // 通过环境变量加载配置
    pub fn load() -> Config {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        
        Config { db_url, secret_key }
    }
}
