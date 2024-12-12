use lazy_static::lazy_static;
use std::sync::Mutex;
lazy_static! {
   // 使用 Mutex 包裹 String，这样可以在需要时修改其值
    pub static ref RABBITMQ_ROUTING_EXCHANGE: Mutex<String> = Mutex::new(String::new());
}
