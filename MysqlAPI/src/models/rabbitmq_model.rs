use lapin::{options::*, types::FieldTable, Channel, Connection, ConnectionProperties};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
#[derive(Clone)] // 自动为 RabbitMQ 实现 Clone
pub struct RabbitMQ {
    pub connection: Arc<Connection>,        //保证 线性安全
    pub channel: Arc<Channel>,              //线性安全
    pub exchange_declared: Arc<AtomicBool>, // 新增字段
}
