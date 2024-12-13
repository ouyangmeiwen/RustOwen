use crate::models::rabbitmq_model::RabbitMQ;
use async_std::prelude::StreamExt;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

impl RabbitMQ {
    // 创建新的 RabbitMQ 实例
    pub async fn new(uri: &str) -> Result<Self, lapin::Error> {
        let connection: Connection =
            Connection::connect(uri, ConnectionProperties::default()).await?;
        let channel: Channel = connection.create_channel().await?;
        Ok(RabbitMQ {
            connection: Arc::new(connection),
            channel: Arc::new(channel),
            exchange_declared: Arc::new(AtomicBool::new(false)), // 初始化标志
        })
    }
    // 声明交换机
    pub async fn declare_exchange(&self, exchange: &str) -> Result<(), lapin::Error> {
        // 检查交换机是否已经声明
        if self.exchange_declared.load(Ordering::SeqCst) {
            return Ok(());
        }
        self.channel
            .exchange_declare(
                exchange,
                lapin::ExchangeKind::Topic,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                eprintln!("Failed to declare exchange: {}", e);
                e
            })?;
        // 设置标记为已声明
        self.exchange_declared.store(true, Ordering::SeqCst);
        Ok(())
    }
    // 发布消息
    pub async fn publish(
        &self,
        exchange: &str,
        routing_key: &str,
        message: &str,
    ) -> Result<(), lapin::Error> {
        self.declare_exchange(exchange).await?;
        self.channel
            .basic_publish(
                exchange,
                routing_key,
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await
            .map_err(|e| {
                eprintln!("Failed to publish message: {}", e);
                e
            })?;
        Ok(())
    }

    // 订阅消息
    pub async fn consume(
        &self,
        exchange: &str,
        queue: &str,
        routing_key: &str,
        tx: mpsc::Sender<String>,
    ) -> Result<(), lapin::Error> {
        self.declare_exchange(exchange).await?;

        // 声明队列
        let queue = self
            .channel
            .queue_declare(queue, QueueDeclareOptions::default(), FieldTable::default())
            .await
            .map_err(|e| {
                eprintln!("Failed to declare queue: {}", e);
                e
            })?;

        // 绑定队列到交换机
        self.channel
            .queue_bind(
                queue.name().as_str(),
                exchange,
                routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                eprintln!("Failed to bind queue: {}", e);
                e
            })?;

        // 开始消费
        let mut consumer = self
            .channel
            .basic_consume(
                queue.name().as_str(),
                "consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                eprintln!("Failed to start consumer: {}", e);
                e
            })?;

        tokio::spawn(async move {
            while let Some(delivery) = consumer.next().await {
                if let Ok(delivery) = delivery {
                    let message = String::from_utf8_lossy(&delivery.data).to_string();
                    // 打印接收到的消息
                    println!("Received message: {}", message);

                    let _ = tx.send(message).await;

                    if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                        eprintln!("Failed to ack message: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// 尝试连接 RabbitMQ 服务，并在失败时重试，最多重试 10 次
    async fn connect_with_retry(uri: &str) -> Option<Connection> {
        let max_retries = 10;
        let mut retries = 0;
        loop {
            match Connection::connect(uri, ConnectionProperties::default()).await {
                Ok(conn) => return Some(conn), // 成功连接，返回连接
                Err(e) => {
                    retries += 1;
                    eprintln!(
                        "Failed to connect to RabbitMQ (attempt {} of {}): {}, retrying...",
                        retries, max_retries, e
                    );
                    // 如果已重试最大次数，则打印错误并返回 None
                    if retries >= max_retries {
                        eprintln!("Failed to connect to RabbitMQ after {} attempts.", retries);
                        return None;
                    }
                    sleep(Duration::from_secs(5)).await; // 每 5 秒重试一次
                }
            }
        }
    }
    /// 断线重连方法
    pub async fn reconnect(&mut self, uri: &str, exchange: &str) -> Result<(), lapin::Error> {
        loop {
            // 尝试重新连接 RabbitMQ
            if let Some(connection) = Self::connect_with_retry(uri).await {
                let channel = connection.create_channel().await?;
                // 更新 RabbitMQ 实例中的连接和通道
                self.connection = Arc::new(connection);
                self.channel = Arc::new(channel);
                self.exchange_declared = Arc::new(AtomicBool::new(false));
                // 重新声明交换机等必要的配置
                self.declare_exchange(exchange).await?; // 假设你需要重新声明一个交换机
                println!("Reconnected to RabbitMQ!");
                return Ok(());
            } else {
                println!("Error to RabbitMQ!");
            }
        }
    }
}
