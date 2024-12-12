use lapin::{options::*, types::FieldTable, Channel, Connection, ConnectionProperties};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Clone)] // 自动为 RabbitMQ 实现 Clone
pub struct RabbitMQ {
    pub connection: Arc<Connection>,
    pub channel: Arc<Channel>,
}
