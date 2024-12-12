use crate::models::rabbitmq_model::RabbitMQ;
use async_std::prelude::StreamExt;
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
};
use std::sync::Arc;
use tokio::sync::mpsc;

impl RabbitMQ {
    // 创建新的 RabbitMQ 实例
    pub async fn new(uri: &str) -> Result<Self, lapin::Error> {
        let connection: Connection = Connection::connect(uri, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        Ok(RabbitMQ {
            connection: Arc::new(connection),
            channel: Arc::new(channel),
        })
    }

    // 声明交换机
    pub async fn declare_exchange(&self, exchange: &str) -> Result<(), lapin::Error> {
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
            })
    }

    // 发布消息
    pub async fn publish(
        &self,
        exchange: &str,
        routing_key: &str,
        message: &str,
    ) -> Result<(), lapin::Error> {
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
                    let _ = tx.send(message).await;

                    if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                        eprintln!("Failed to ack message: {}", e);
                    }
                }
            }
        });

        Ok(())
    }
}
