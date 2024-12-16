use lazy_static::lazy_static;
use std::sync::RwLock;
use std::{collections::HashMap, sync::Mutex};
lazy_static! {
   // 使用 Mutex 包裹 String，这样可以在需要时修改其值
    pub static ref RABBITMQ_ROUTING_EXCHANGE: RwLock<String> = RwLock::new(String::new());// 使用 Mutex 包裹 i32，这样可以在需要时修改其值
    pub static ref GLOBAL_NUMBER: RwLock<i32> = RwLock::new(0);
}
